use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    domain::{
        reference::SearchFilter,
        repo::{RepoProfile, RepoSearchResult},
    },
    services::repos::{RepoSearchFilters, get_repo_profile, search_github_repos},
};

#[derive(Debug, Deserialize)]
pub struct RepoSearchQuery {
    pub q: Option<String>,
    #[serde(default)]
    pub filter: SearchFilter,
    pub language: Option<String>,
    pub license: Option<String>,
    pub stars_min: Option<i32>,
    #[serde(default)]
    pub include_archived: bool,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSearchResponse {
    pub filter: SearchFilter,
    pub items: Vec<RepoSearchResult>,
}

pub async fn search_repos(
    State(state): State<AppState>,
    Query(query): Query<RepoSearchQuery>,
) -> Result<Json<RepoSearchResponse>, ApiError> {
    let filters = RepoSearchFilters {
        query: normalize(query.q),
        filter: query.filter,
        language: normalize(query.language),
        license_spdx: normalize(query.license),
        stars_min: query.stars_min.filter(|v| *v >= 0),
        include_archived: query.include_archived,
        limit: query.limit,
    };
    let items = search_github_repos(&state.db, &filters).await?;
    Ok(Json(RepoSearchResponse {
        filter: filters.filter,
        items,
    }))
}

pub async fn get_repo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RepoProfile>, ApiError> {
    let profile = get_repo_profile(&state.db, id).await?;
    Ok(Json(profile))
}

fn normalize(value: Option<String>) -> Option<String> {
    value.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}
