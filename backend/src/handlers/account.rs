use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};

use crate::{
    app::{AppState, error::ApiError},
    auth::{clear_session_cookie, resolve_current_user},
    domain::account::AccountSummary,
    services::{
        account_deletion::{DeleteAccountOutcome, delete_account_data},
        account_preferences::{self, NotificationPreferences, UpdateNotificationPreferences},
        trust::reputation,
    },
};

pub async fn account_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AccountSummary>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let reputation = reputation::get_user_reputation(&state.db, user.id).await?;

    Ok(Json(AccountSummary {
        reputation: reputation.to_summary(state.config.active_signal_min_reputation),
        active_signal_min_reputation: state.config.active_signal_min_reputation,
        active_signal_default_consensus: state.config.active_signal_default_consensus,
        active_signal_severe_consensus: state.config.active_signal_severe_consensus,
    }))
}

pub async fn get_notification_preferences(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<NotificationPreferences>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let preferences = account_preferences::get(&state.db, user.id).await?;
    Ok(Json(preferences))
}

pub async fn update_notification_preferences(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdateNotificationPreferences>,
) -> Result<Json<NotificationPreferences>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let preferences = account_preferences::update(&state.db, user.id, req).await?;
    Ok(Json(preferences))
}

pub async fn delete_account(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let outcome: DeleteAccountOutcome = delete_account_data(&state.db, user.id).await?;
    let cookie = clear_session_cookie(&state.config)?;
    let mut response = (StatusCode::OK, Json(outcome)).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|_| ApiError::bad_request("invalid session cookie"))?,
    );
    Ok(response)
}
