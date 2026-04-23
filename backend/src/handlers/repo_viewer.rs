use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::repo::RepoSignal,
    services::{repos::get_repo_signals, trust::repo_owners},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoViewerState {
    pub can_dispute_signals: bool,
    pub visible_signals: Vec<RepoSignal>,
}

pub async fn get_repo_viewer_state(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(repo_id): Path<Uuid>,
) -> Result<Json<RepoViewerState>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let can_dispute =
        repo_owners::user_can_manage_repo_signal(&state.db, &state.config, user.id, repo_id)
            .await?;
    let signals = get_repo_signals(&state.db, repo_id).await?;

    Ok(Json(RepoViewerState {
        can_dispute_signals: can_dispute,
        visible_signals: if can_dispute {
            signals
        } else {
            signals
                .into_iter()
                .filter(|signal| signal.is_passive || signal.review_status == "accepted")
                .collect()
        },
    }))
}
