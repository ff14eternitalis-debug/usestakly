use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::watchlist::{AddWatchRequest, WatchedRepo},
    services::{quality::load_v2, watchlist},
};

pub async fn list_watchlist(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<WatchedRepo>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let formula_version = load_v2()?.meta.version;
    let items = watchlist::list_for_user(&state.db, user.id, &formula_version).await?;
    Ok(Json(items))
}

pub async fn add_to_watchlist(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<AddWatchRequest>,
) -> Result<(StatusCode, Json<WatchedRef>), ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let id = watchlist::add_watch(&state.db, user.id, payload.external_artifact_id).await?;
    Ok((
        StatusCode::CREATED,
        Json(WatchedRef {
            id,
            external_artifact_id: payload.external_artifact_id,
        }),
    ))
}

pub async fn remove_from_watchlist(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(artifact_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    watchlist::remove_watch(&state.db, user.id, artifact_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_watch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(artifact_id): Path<Uuid>,
    Json(payload): Json<UpdateWatchRequest>,
) -> Result<StatusCode, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    watchlist::set_muted(&state.db, user.id, artifact_id, payload.muted).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct UpdateWatchRequest {
    pub muted: bool,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchedRef {
    pub id: Uuid,
    pub external_artifact_id: Uuid,
}
