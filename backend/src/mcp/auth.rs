use http::{HeaderValue, request::Parts};
use rmcp::ErrorData;
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::agent_tokens::{self, VerifiedAgent};

/// Verify a Bearer token from the request's Authorization header against `agent_tokens`.
/// Returns the authenticated user's UUID. Every MCP tool must call this first.
pub async fn verify_bearer(db: &PgPool, parts: &Parts) -> Result<Uuid, ErrorData> {
    Ok(verify_agent(db, parts).await?.user_id)
}

pub async fn verify_agent(db: &PgPool, parts: &Parts) -> Result<VerifiedAgent, ErrorData> {
    let header: &HeaderValue = parts
        .headers
        .get(http::header::AUTHORIZATION)
        .ok_or_else(|| ErrorData::invalid_request("missing Authorization header", None))?;
    let raw = header
        .to_str()
        .map_err(|_| ErrorData::invalid_request("malformed Authorization header", None))?;
    let token = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
        .ok_or_else(|| ErrorData::invalid_request("expected 'Bearer <token>'", None))?;
    agent_tokens::verify(db, token.trim())
        .await
        .map_err(|_| ErrorData::invalid_request("invalid or revoked token", None))
}
