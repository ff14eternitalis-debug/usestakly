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
        repos::{
            IndexedRepoAddRow, find_github_artifact_id, formula_version_for_add,
            load_indexed_repo_for_add, should_short_circuit_github_ingest,
        },
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

    if let Some(artifact_id) = find_github_artifact_id(&state.db, &owner, &name).await? {
        let row = load_indexed_repo_for_add(&state.db, artifact_id).await?;
        let formula_version = formula_version_for_add()?;
        return Ok(Json(add_response_from_row(row, true, formula_version)));
    }

    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("Repo ingestion disabled (set GITHUB_TOKEN)"))?;

    let client = build_client(token)?;
    let (artifact_id, meta, categories) =
        ingest_repo(&client, &state.db, &state.config, &owner, &name).await?;
    recompute_external_artifact(&state.db, Some(&state.config), artifact_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    let formula_version = formula_version_for_add()?;

    Ok(Json(AddRepoResponse {
        artifact_id,
        already_indexed: false,
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

fn add_response_from_row(
    row: IndexedRepoAddRow,
    already_indexed: bool,
    formula_version: String,
) -> AddRepoResponse {
    debug_assert!(
        !already_indexed || should_short_circuit_github_ingest(already_indexed),
        "indexed path must not call GitHub"
    );
    AddRepoResponse {
        artifact_id: row.artifact_id,
        already_indexed,
        owner: row.owner.clone(),
        name: row.name.clone(),
        full_name: format!("{}/{}", row.owner, row.name),
        html_url: row.html_url,
        description: row.description,
        language: row.language,
        license_spdx: row.license_spdx,
        topics: row.topics,
        stars_count: row.stars_count,
        forks_count: row.forks_count,
        open_issues_count: row.open_issues_count,
        subscribers_count: row.subscribers_count,
        archived: row.archived,
        default_branch: row.default_branch,
        last_commit_at: row.last_commit_at,
        formula_version,
        categories: row.categories.0,
    }
}

#[cfg(test)]
mod tests {
    use super::add_response_from_row;
    use crate::services::repos::{IndexedRepoAddRow, should_short_circuit_github_ingest};
    use chrono::Utc;
    use sqlx::types::Json;
    use uuid::Uuid;

    fn sample_row() -> IndexedRepoAddRow {
        IndexedRepoAddRow {
            artifact_id: Uuid::nil(),
            owner: "FFmpeg".to_string(),
            name: "FFmpeg".to_string(),
            html_url: "https://github.com/FFmpeg/FFmpeg".to_string(),
            description: None,
            language: Some("C".to_string()),
            license_spdx: None,
            topics: vec![],
            stars_count: 1,
            forks_count: 0,
            open_issues_count: 0,
            subscribers_count: 0,
            archived: false,
            default_branch: Some("master".to_string()),
            last_commit_at: Some(Utc::now()),
            categories: Json(vec![]),
        }
    }

    #[test]
    fn indexed_response_marks_already_indexed_without_ingest_path() {
        let resp = add_response_from_row(sample_row(), true, "v2.0".to_string());
        assert!(resp.already_indexed);
        assert!(should_short_circuit_github_ingest(true));
        assert_eq!(resp.owner, "FFmpeg");
    }
}
