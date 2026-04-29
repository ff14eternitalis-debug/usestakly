use axum::{Json, extract::State, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    services::{
        recommendations::{UseCaseRecommendationReport, recommend_for_use_case},
        use_case_watches::{UseCaseWatch, create_watch, list_watches},
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendUseCaseRequest {
    pub query: String,
    #[serde(default = "default_risk_tolerance")]
    pub risk_tolerance: String,
    pub limit: Option<i64>,
}

pub async fn recommend_use_case(
    State(state): State<AppState>,
    Json(req): Json<RecommendUseCaseRequest>,
) -> Result<Json<UseCaseRecommendationReport>, ApiError> {
    let query = req.query.trim();
    if query.is_empty() {
        return Err(ApiError::bad_request("Query must not be empty"));
    }
    let report = recommend_for_use_case(
        &state.db,
        &state.config,
        query,
        &req.risk_tolerance,
        req.limit.unwrap_or(8),
    )
    .await?;
    Ok(Json(report))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUseCaseWatchRequest {
    pub query: String,
    pub label: Option<String>,
    #[serde(default = "default_risk_tolerance")]
    pub risk_tolerance: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUseCaseWatchResponse {
    pub watch_id: Uuid,
    pub watch: UseCaseWatch,
}

pub async fn create_use_case_watch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateUseCaseWatchRequest>,
) -> Result<Json<CreateUseCaseWatchResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let query = req.query.trim();
    if query.is_empty() {
        return Err(ApiError::bad_request("Query must not be empty"));
    }
    let watch = create_watch(
        &state.db,
        &state.config,
        user.id,
        query,
        req.label,
        &req.risk_tolerance,
    )
    .await?;
    Ok(Json(CreateUseCaseWatchResponse {
        watch_id: watch.id,
        watch,
    }))
}

pub async fn list_use_case_watches(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<UseCaseWatch>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let watches = list_watches(&state.db, user.id).await?;
    Ok(Json(watches))
}

fn default_risk_tolerance() -> String {
    "medium".to_string()
}
