use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, domain::quality::QualitySignalRecord};

pub async fn review_signal(
    db: &PgPool,
    signal_id: Uuid,
    status: &str,
    reviewed_by_user_id: Option<Uuid>,
    note: Option<&str>,
) -> Result<QualitySignalRecord, ApiError> {
    let record = sqlx::query_as::<_, QualitySignalRecord>(
        r#"
        UPDATE quality_signals
        SET
          review_status = $2,
          reviewed_by_user_id = $3,
          reviewed_at = $4,
          review_note = $5,
          disputed_by_user_id = CASE WHEN $2 = 'accepted' THEN NULL ELSE disputed_by_user_id END,
          disputed_at = CASE WHEN $2 = 'accepted' THEN NULL ELSE disputed_at END,
          dispute_reason = CASE WHEN $2 = 'accepted' THEN NULL ELSE dispute_reason END
        WHERE id = $1
        RETURNING
          id,
          artifact_kind::text AS artifact_kind,
          snippet_id,
          external_artifact_id,
          signal::text AS signal,
          is_passive,
          actor_user_id,
          agent_context,
          evidence_url,
          evidence_description,
          review_status,
          reviewed_by_user_id,
          reviewed_at,
          review_note,
          disputed_by_user_id,
          disputed_at,
          dispute_reason,
          created_at
        "#,
    )
    .bind(signal_id)
    .bind(status)
    .bind(reviewed_by_user_id)
    .bind(Utc::now())
    .bind(note)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Signal not found"))?;

    Ok(record)
}

pub async fn dispute_signal(
    db: &PgPool,
    signal_id: Uuid,
    disputed_by_user_id: Uuid,
    reason: &str,
) -> Result<QualitySignalRecord, ApiError> {
    let record = sqlx::query_as::<_, QualitySignalRecord>(
        r#"
        UPDATE quality_signals
        SET
          review_status = CASE WHEN review_status = 'accepted' THEN review_status ELSE 'disputed' END,
          disputed_by_user_id = $2,
          disputed_at = $3,
          dispute_reason = $4
        WHERE id = $1
          AND is_passive = FALSE
        RETURNING
          id,
          artifact_kind::text AS artifact_kind,
          snippet_id,
          external_artifact_id,
          signal::text AS signal,
          is_passive,
          actor_user_id,
          agent_context,
          evidence_url,
          evidence_description,
          review_status,
          reviewed_by_user_id,
          reviewed_at,
          review_note,
          disputed_by_user_id,
          disputed_at,
          dispute_reason,
          created_at
        "#,
    )
    .bind(signal_id)
    .bind(disputed_by_user_id)
    .bind(Utc::now())
    .bind(reason)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Signal not found"))?;

    Ok(record)
}

pub async fn signal_belongs_to_repo(
    db: &PgPool,
    repo_id: Uuid,
    signal_id: Uuid,
) -> Result<bool, ApiError> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM quality_signals
          WHERE id = $1
            AND external_artifact_id = $2
        )
        "#,
    )
    .bind(signal_id)
    .bind(repo_id)
    .fetch_one(db)
    .await?;
    Ok(exists)
}
