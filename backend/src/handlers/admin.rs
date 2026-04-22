use axum::{Json, extract::State, http::HeaderMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::{ScoringReport, recompute_all_scores},
    },
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

#[derive(Deserialize)]
pub struct IngestGithubRequest {
    pub owner: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct IngestGithubResponse {
    pub id: Uuid,
    pub owner: String,
    pub name: String,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub default_branch: Option<String>,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub topics: Vec<String>,
}

pub async fn ingest_github_repo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<IngestGithubRequest>,
) -> Result<Json<IngestGithubResponse>, ApiError> {
    require_admin_token(&state, &headers)?;

    let owner = req.owner.trim();
    let name = req.name.trim();
    if owner.is_empty() || name.is_empty() {
        return Err(ApiError::bad_request("owner and name are required"));
    }
    if owner.contains('/') || name.contains('/') || owner.contains(' ') || name.contains(' ') {
        return Err(ApiError::bad_request(
            "owner and name must not contain '/' or whitespace",
        ));
    }

    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("GitHub ingestion disabled (set GITHUB_TOKEN)"))?;

    let client = build_client(token)?;
    let (id, meta) = ingest_repo(&client, &state.db, owner, name).await?;

    Ok(Json(IngestGithubResponse {
        id,
        owner: meta.owner,
        name: meta.name,
        stars_count: meta.stars_count,
        forks_count: meta.forks_count,
        open_issues_count: meta.open_issues_count,
        subscribers_count: meta.subscribers_count,
        archived: meta.archived,
        language: meta.language,
        license_spdx: meta.license_spdx,
        default_branch: meta.default_branch,
        last_commit_at: meta.last_commit_at,
        topics: meta.topics,
    }))
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
