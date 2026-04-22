use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::watchlist::Notification,
    services::notifications,
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 200;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default)]
    pub unread: bool,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnreadCount {
    pub unread: i64,
}

pub async fn list_notifications(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Notification>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let items = notifications::list_for_user(&state.db, user.id, query.unread, limit).await?;
    Ok(Json(items))
}

pub async fn unread_count(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UnreadCount>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let unread = notifications::unread_count(&state.db, user.id).await?;
    Ok(Json(UnreadCount { unread }))
}

pub async fn mark_notification_read(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(notification_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    notifications::mark_read(&state.db, user.id, notification_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkAllReadResponse {
    pub updated: u64,
}

pub async fn mark_all_read(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<MarkAllReadResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let updated = notifications::mark_all_read(&state.db, user.id).await?;
    Ok(Json(MarkAllReadResponse { updated }))
}
