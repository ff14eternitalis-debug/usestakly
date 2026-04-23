use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::agent_token::{AgentTokenCreated, AgentTokenSummary},
    services::agent_tokens,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAgentTokenRequest {
    #[validate(length(min = 1, max = 80))]
    pub label: String,
}

pub async fn create_agent_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateAgentTokenRequest>,
) -> Result<(StatusCode, Json<AgentTokenCreated>), ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    req.validate()
        .map_err(|e| ApiError::bad_request(format!("validation: {e}")))?;
    let created = agent_tokens::create(&state.db, user.id, &req.label).await?;
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn list_agent_tokens(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AgentTokenSummary>>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let items = agent_tokens::list_for_user(&state.db, user.id).await?;
    Ok(Json(items))
}

pub async fn revoke_agent_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(token_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    agent_tokens::revoke(&state.db, user.id, token_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
