use axum::{Json, extract::State, http::HeaderMap};

use crate::{
    app::{AppState, error::ApiError},
    auth::{CurrentUser, resolve_current_user},
};

pub async fn me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<CurrentUser>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    Ok(Json(user))
}
