use axum::{Json, extract::State, http::HeaderMap};

use crate::{
    app::{AppState, error::ApiError},
    services::quality::{ScoringReport, recompute_all_scores},
};

const ADMIN_TOKEN_HEADER: &str = "x-admin-token";

pub async fn recompute_scores(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ScoringReport>, ApiError> {
    require_admin_token(&state, &headers)?;
    let report = recompute_all_scores(&state.db).await?;
    Ok(Json(report))
}

fn require_admin_token(state: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    let expected = state
        .config
        .admin_api_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("Admin API not enabled (set ADMIN_API_TOKEN)"))?;
    let provided = headers
        .get(ADMIN_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::forbidden("Missing admin token"))?;
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return Err(ApiError::forbidden("Invalid admin token"));
    }
    Ok(())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
