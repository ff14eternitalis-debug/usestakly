use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::quality::SignalKind,
};

const EVENT_LOG_USAGE: &str = "mcp_log_usage";
const EVENT_WATCH_REPO: &str = "mcp_watch_repo";

pub async fn enforce_write_quota(
    db: &PgPool,
    token_id: Uuid,
    max_per_hour: u32,
) -> Result<(), ApiError> {
    let used: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM agent_token_events
          WHERE token_id = $1
            AND created_at >= NOW() - INTERVAL '1 hour'
            AND kind IN ('mcp_log_usage', 'mcp_watch_repo')
        "#,
    )
    .bind(token_id)
    .fetch_one(db)
    .await?;

    if used >= i64::from(max_per_hour) {
        return Err(ApiError::too_many_requests(format!(
            "MCP write quota exceeded for this token ({max_per_hour}/hour)"
        )));
    }

    Ok(())
}

pub async fn enforce_log_usage_guards(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
    outcome: SignalKind,
    cooldown_secs: u64,
    negative_window_hours: u64,
) -> Result<(), ApiError> {
    let cooldown_secs = i32::try_from(cooldown_secs)
        .map_err(|_| ApiError::bad_request("cooldown is too large"))?;
    let duplicate_recent: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM agent_token_events
          WHERE token_id = $1
            AND kind = $2
            AND lower(repo_owner) = lower($3)
            AND lower(repo_name) = lower($4)
            AND payload->>'outcome' = $5
            AND created_at >= NOW() - make_interval(secs => $6)
        )
        "#,
    )
    .bind(token_id)
    .bind(EVENT_LOG_USAGE)
    .bind(owner)
    .bind(name)
    .bind(outcome.as_str())
    .bind(cooldown_secs)
    .fetch_one(db)
    .await?;

    if duplicate_recent {
        return Err(ApiError::too_many_requests(format!(
            "duplicate MCP usage signal blocked for {owner}/{name}; retry later"
        )));
    }

    if !is_negative_outcome(outcome) {
        return Ok(());
    }

    let negative_window_hours = i32::try_from(negative_window_hours)
        .map_err(|_| ApiError::bad_request("negative window is too large"))?;
    let recent_negative: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM agent_token_events
          WHERE user_id = $1
            AND kind = $2
            AND lower(repo_owner) = lower($3)
            AND lower(repo_name) = lower($4)
            AND payload->>'outcome' IN ('build_failure', 'regret', 're_resolve')
            AND created_at >= NOW() - make_interval(hours => $5)
        )
        "#,
    )
    .bind(user_id)
    .bind(EVENT_LOG_USAGE)
    .bind(owner)
    .bind(name)
    .bind(negative_window_hours)
    .fetch_one(db)
    .await?;

    if recent_negative {
        return Err(ApiError::too_many_requests(format!(
            "negative MCP usage signals for {owner}/{name} are limited; retry later"
        )));
    }

    Ok(())
}

pub async fn record_log_usage(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
    outcome: SignalKind,
    notes: Option<&str>,
) -> Result<(), ApiError> {
    record_event(
        db,
        token_id,
        user_id,
        EVENT_LOG_USAGE,
        owner,
        name,
        serde_json::json!({
            "outcome": outcome.as_str(),
            "notes": notes,
        }),
    )
    .await
}

pub async fn record_watch_repo(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
) -> Result<(), ApiError> {
    record_event(
        db,
        token_id,
        user_id,
        EVENT_WATCH_REPO,
        owner,
        name,
        serde_json::json!({}),
    )
    .await
}

async fn record_event(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    kind: &str,
    owner: &str,
    name: &str,
    payload: Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO agent_token_events (token_id, user_id, kind, repo_owner, repo_name, payload)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(token_id)
    .bind(user_id)
    .bind(kind)
    .bind(owner)
    .bind(name)
    .bind(payload)
    .execute(db)
    .await?;
    Ok(())
}

fn is_negative_outcome(outcome: SignalKind) -> bool {
    matches!(
        outcome,
        SignalKind::BuildFailure | SignalKind::Regret | SignalKind::ReResolve
    )
}
