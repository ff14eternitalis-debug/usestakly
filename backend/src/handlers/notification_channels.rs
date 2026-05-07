use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    services::notification_channels::{
        self, NotificationChannelSummary, UpsertNotificationChannel,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNotificationChannelResponse {
    pub ok: bool,
}

pub async fn list_notification_channels(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<NotificationChannelSummary>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let channels = notification_channels::list_for_user(&state.db, user.id).await?;
    Ok(Json(channels))
}

pub async fn upsert_notification_channel(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpsertNotificationChannel>,
) -> Result<(StatusCode, Json<NotificationChannelSummary>), ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let secret = state
        .config
        .notification_secret()
        .ok_or_else(|| ApiError::internal("APP_NOTIFICATION_SECRET is required"))?;
    let channel = notification_channels::upsert(&state.db, user.id, secret, req).await?;
    Ok((StatusCode::CREATED, Json(channel)))
}

pub async fn delete_notification_channel(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(channel_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    notification_channels::delete(&state.db, user.id, channel_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn test_notification_channel(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<TestNotificationChannelResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let secret = state
        .config
        .notification_secret()
        .ok_or_else(|| ApiError::internal("APP_NOTIFICATION_SECRET is required"))?;
    notification_channels::send_test(&state.db, user.id, secret, channel_id).await?;
    Ok(Json(TestNotificationChannelResponse { ok: true }))
}
