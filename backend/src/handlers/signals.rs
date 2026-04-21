use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::quality::{ArtifactKind, CreateSignalRequest, QualitySignalRecord},
    services::quality::{RecordSignalInput, record_signal},
};

pub async fn create_snippet_signal(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(snippet_id): Path<Uuid>,
    Json(payload): Json<CreateSignalRequest>,
) -> Result<Json<QualitySignalRecord>, ApiError> {
    payload.validate()?;
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;

    if payload.signal.is_passive() {
        return Err(ApiError::bad_request(
            "Passive signals are emitted by the MCP resolver, not via this endpoint",
        ));
    }

    ensure_snippet_exists(&state, snippet_id).await?;

    let record = record_signal(
        &state.db,
        RecordSignalInput {
            artifact_kind: ArtifactKind::Snippet,
            snippet_id: Some(snippet_id),
            external_artifact_id: None,
            signal: payload.signal,
            actor_user_id: Some(user.id),
            evidence_url: payload.evidence_url,
            evidence_description: payload.evidence_description,
            agent_context: payload.agent_context,
        },
    )
    .await?;

    Ok(Json(record))
}

async fn ensure_snippet_exists(state: &AppState, snippet_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM snippets WHERE id = $1 AND visibility = 'public'",
    )
    .bind(snippet_id)
    .fetch_one(&state.db)
    .await?;

    if exists == 0 {
        return Err(ApiError::not_found(
            "Snippet not found or not publicly signalable",
        ));
    }

    Ok(())
}
