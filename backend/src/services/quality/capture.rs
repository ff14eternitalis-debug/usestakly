use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    domain::quality::{ArtifactKind, QualitySignalRecord, SignalKind},
};

pub struct RecordSignalInput {
    pub artifact_kind: ArtifactKind,
    pub snippet_id: Option<Uuid>,
    pub external_artifact_id: Option<Uuid>,
    pub signal: SignalKind,
    pub actor_user_id: Option<Uuid>,
    pub evidence_url: Option<String>,
    pub evidence_description: Option<String>,
    pub agent_context: Option<Value>,
}

pub async fn record_signal(
    db: &PgPool,
    input: RecordSignalInput,
) -> Result<QualitySignalRecord, ApiError> {
    validate_artifact_reference(&input)?;
    validate_evidence(&input)?;

    let record = sqlx::query_as::<_, QualitySignalRecord>(
        r#"
        INSERT INTO quality_signals (
          artifact_kind,
          snippet_id,
          external_artifact_id,
          signal,
          is_passive,
          actor_user_id,
          agent_context,
          evidence_url,
          evidence_description
        )
        VALUES (
          CAST($1 AS artifact_kind),
          $2,
          $3,
          CAST($4 AS signal_kind),
          $5,
          $6,
          $7,
          $8,
          $9
        )
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
          created_at
        "#,
    )
    .bind(input.artifact_kind.as_str())
    .bind(input.snippet_id)
    .bind(input.external_artifact_id)
    .bind(input.signal.as_str())
    .bind(input.signal.is_passive())
    .bind(input.actor_user_id)
    .bind(input.agent_context)
    .bind(input.evidence_url)
    .bind(input.evidence_description)
    .fetch_one(db)
    .await?;

    Ok(record)
}

fn validate_artifact_reference(input: &RecordSignalInput) -> Result<(), ApiError> {
    match input.artifact_kind {
        ArtifactKind::Snippet => {
            if input.snippet_id.is_none() || input.external_artifact_id.is_some() {
                return Err(ApiError::bad_request(
                    "artifact_kind=snippet requires snippet_id only",
                ));
            }
        }
        ArtifactKind::External => {
            if input.external_artifact_id.is_none() || input.snippet_id.is_some() {
                return Err(ApiError::bad_request(
                    "artifact_kind=external requires external_artifact_id only",
                ));
            }
        }
    }
    Ok(())
}

fn validate_evidence(input: &RecordSignalInput) -> Result<(), ApiError> {
    if input.signal.is_passive() {
        return Ok(());
    }
    if input.signal.requires_evidence()
        && input.evidence_url.is_none()
        && input.evidence_description.is_none()
    {
        return Err(ApiError::bad_request(format!(
            "Signal '{}' requires evidence_url or evidence_description",
            input.signal.as_str()
        )));
    }
    Ok(())
}
