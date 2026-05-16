use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    domain::repo::RepoCategory,
    services::{
        ingestion::github::{build_client, ingest_repo, parse_github_repo_input},
        quality::recompute_external_artifact,
        repos::find_github_artifact_id,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRepoRequest {
    pub repo: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRepoResponse {
    pub artifact_id: Uuid,
    pub already_indexed: bool,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub default_branch: Option<String>,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub formula_version: String,
    pub categories: Vec<RepoCategory>,
}

pub async fn add_repo(
    State(state): State<AppState>,
    Json(payload): Json<AddRepoRequest>,
) -> Result<Json<AddRepoResponse>, ApiError> {
    let (owner, name) = parse_github_repo_input(&payload.repo)?;
    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("Repo ingestion disabled (set GITHUB_TOKEN)"))?;

    let already_indexed = find_github_artifact_id(&state.db, &owner, &name)
        .await?
        .is_some();
    let client = build_client(token)?;
    let (artifact_id, meta, categories) =
        ingest_repo(&client, &state.db, &state.config, &owner, &name).await?;
    recompute_external_artifact(&state.db, Some(&state.config), artifact_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    let formula_version = crate::services::quality::load_v2()
        .map_err(|e| ApiError::internal(format!("loading formula: {e}")))?
        .meta
        .version;

    Ok(Json(AddRepoResponse {
        artifact_id,
        already_indexed,
        owner: meta.owner.clone(),
        name: meta.name.clone(),
        full_name: format!("{}/{}", meta.owner, meta.name),
        html_url: meta.html_url,
        description: meta.description,
        language: meta.language,
        license_spdx: meta.license_spdx,
        topics: meta.topics,
        stars_count: meta.stars_count,
        forks_count: meta.forks_count,
        open_issues_count: meta.open_issues_count,
        subscribers_count: meta.subscribers_count,
        archived: meta.archived,
        default_branch: meta.default_branch,
        last_commit_at: meta.last_commit_at,
        formula_version,
        categories,
    }))
}
