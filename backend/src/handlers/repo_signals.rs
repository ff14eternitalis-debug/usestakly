use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::quality::{CreateSignalRequest, SignalKind},
    services::{
        quality::{RecordSignalInput, recompute_all_scores_with_config, record_signal},
        trust::{repo_owners, reputation, signal_events, signal_reviews},
    },
};

#[derive(Debug, serde::Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DisputeSignalRequest {
    #[validate(length(min = 10, max = 2000))]
    pub reason: String,
}

pub async fn create_repo_signal(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(repo_id): Path<Uuid>,
    Json(payload): Json<CreateSignalRequest>,
) -> Result<Json<crate::domain::quality::QualitySignalRecord>, ApiError> {
    if payload.signal.is_passive() {
        return Err(ApiError::bad_request(
            "Only active signals can be submitted on repos",
        ));
    }

    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let reputation = reputation::get_user_reputation(&state.db, user.id).await?;
    if !reputation.active_signal_eligible(state.config.active_signal_min_reputation) {
        return Err(ApiError::forbidden(format!(
            "Active signals require a reputation >= {:.2}, at least 5 passive signals, and a 7-day-old account",
            state.config.active_signal_min_reputation
        )));
    }
    let strict_review = matches!(
        payload.signal,
        SignalKind::SecurityIssue | SignalKind::Broken | SignalKind::DoesntMatchClaim
    ) && reputation.requires_strict_active_review();
    let review_status = if matches!(payload.signal, SignalKind::SecurityIssue) || strict_review {
        "pending".to_string()
    } else {
        "accepted".to_string()
    };
    let submitted_note = if strict_review {
        Some(format!(
            "reporter-tier={} score={:.2} usage={} strict-review=true",
            reputation.tier.as_str(),
            reputation.score,
            reputation.usage_signal_count()
        ))
    } else {
        Some(format!(
            "reporter-tier={} score={:.2} usage={}",
            reputation.tier.as_str(),
            reputation.score,
            reputation.usage_signal_count()
        ))
    };

    let record = record_signal(
        &state.db,
        RecordSignalInput {
            artifact_kind: crate::domain::quality::ArtifactKind::External,
            snippet_id: None,
            external_artifact_id: Some(repo_id),
            signal: payload.signal,
            review_status,
            actor_user_id: Some(user.id),
            evidence_url: payload.evidence_url,
            evidence_description: payload.evidence_description,
            agent_context: payload.agent_context,
        },
    )
    .await?;
    signal_events::record_signal_event(
        &state.db,
        record.id,
        "submitted",
        Some(user.id),
        submitted_note.as_deref(),
    )
    .await?;
    recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;

    Ok(Json(record))
}

pub async fn dispute_repo_signal(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path((repo_id, signal_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<DisputeSignalRequest>,
) -> Result<Json<crate::domain::quality::QualitySignalRecord>, ApiError> {
    payload.validate()?;
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let owner_reputation = reputation::get_user_reputation(&state.db, user.id).await?;

    let can_manage =
        repo_owners::user_can_manage_repo_signal(&state.db, &state.config, user.id, repo_id)
            .await?;
    if !can_manage {
        return Err(ApiError::forbidden(
            "Only the GitHub repo owner can dispute active signals",
        ));
    }
    if !signal_reviews::signal_belongs_to_repo(&state.db, repo_id, signal_id).await? {
        return Err(ApiError::not_found("Signal not found for this repo"));
    }

    let record =
        signal_reviews::dispute_signal(&state.db, signal_id, user.id, payload.reason.trim())
            .await?;
    let dispute_note = format!(
        "owner-tier={} owner-score={:.2} owner-usage={} reason={}",
        owner_reputation.tier.as_str(),
        owner_reputation.score,
        owner_reputation.usage_signal_count(),
        payload.reason.trim()
    );
    signal_events::record_signal_event(
        &state.db,
        record.id,
        "disputed",
        Some(user.id),
        Some(dispute_note.as_str()),
    )
    .await?;
    recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;
    Ok(Json(record))
}
