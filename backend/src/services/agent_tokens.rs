use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::agent_token::{AgentTokenCreated, AgentTokenSummary},
};

#[derive(FromRow)]
struct TokenListRow {
    id: Uuid,
    label: String,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
}

const TOKEN_PREFIX: &str = "usk_";

pub struct VerifiedAgent {
    pub user_id: Uuid,
    pub token_id: Uuid,
}

pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    label: &str,
) -> Result<AgentTokenCreated, ApiError> {
    let label = label.trim();
    if label.is_empty() {
        return Err(ApiError::bad_request("label is required"));
    }
    if label.len() > 80 {
        return Err(ApiError::bad_request("label too long (max 80)"));
    }

    let plaintext = generate_token();
    let hash = hash_token(&plaintext);

    let row: (Uuid, DateTime<Utc>) = sqlx::query_as(
        r#"
        INSERT INTO agent_tokens (user_id, label, token_hash)
        VALUES ($1, $2, $3)
        RETURNING id, created_at
        "#,
    )
    .bind(user_id)
    .bind(label)
    .bind(&hash)
    .fetch_one(db)
    .await?;

    Ok(AgentTokenCreated {
        id: row.0,
        label: label.to_string(),
        token: plaintext,
        created_at: row.1,
    })
}

pub async fn list_for_user(db: &PgPool, user_id: Uuid) -> Result<Vec<AgentTokenSummary>, ApiError> {
    let rows: Vec<TokenListRow> = sqlx::query_as(
        r#"
        SELECT id, label, created_at, last_used_at
        FROM agent_tokens
        WHERE user_id = $1 AND revoked_at IS NULL
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| AgentTokenSummary {
            id: r.id,
            label: r.label,
            created_at: r.created_at,
            last_used_at: r.last_used_at,
        })
        .collect())
}

pub async fn revoke(db: &PgPool, user_id: Uuid, token_id: Uuid) -> Result<(), ApiError> {
    let rows = sqlx::query(
        r#"
        UPDATE agent_tokens
        SET revoked_at = NOW()
        WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL
        "#,
    )
    .bind(token_id)
    .bind(user_id)
    .execute(db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError::not_found("Token not found"));
    }
    Ok(())
}

pub async fn verify(db: &PgPool, plaintext: &str) -> Result<VerifiedAgent, ApiError> {
    let trimmed = plaintext
        .strip_prefix(TOKEN_PREFIX)
        .ok_or_else(|| ApiError::forbidden("invalid token format"))?;
    if trimmed.len() != 64 || !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ApiError::forbidden("invalid token format"));
    }

    let hash = hash_token(plaintext);
    let row: Option<(Uuid, Uuid)> = sqlx::query_as(
        r#"
        SELECT id, user_id
        FROM agent_tokens
        WHERE token_hash = $1 AND revoked_at IS NULL
        "#,
    )
    .bind(&hash)
    .fetch_optional(db)
    .await?;

    let (token_id, user_id) = row.ok_or_else(|| ApiError::forbidden("invalid token"))?;

    // Fire-and-forget last_used_at bump (doesn't block auth)
    let db = db.clone();
    tokio::spawn(async move {
        let _ = sqlx::query("UPDATE agent_tokens SET last_used_at = NOW() WHERE id = $1")
            .bind(token_id)
            .execute(&db)
            .await;
    });

    Ok(VerifiedAgent { user_id, token_id })
}

fn generate_token() -> String {
    // 32 bytes = 256 bits via two Uuid::new_v4() (cryptographically seeded on all
    // supported platforms). Hex-encoded to keep it URL-safe and easy to paste.
    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    let mut bytes = [0u8; 32];
    bytes[..16].copy_from_slice(a.as_bytes());
    bytes[16..].copy_from_slice(b.as_bytes());
    format!("{TOKEN_PREFIX}{}", hex::encode(bytes))
}

fn hash_token(plaintext: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plaintext.as_bytes());
    hex::encode(hasher.finalize())
}
