use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{AppState, error::ApiError},
    domain::{reference::SearchFilter, repo::RepoSearchResult},
    services::repos::{RepoSearchFilters, search_github_repos},
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
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
pub struct SearchResponse {
    pub filter: SearchFilter,
    pub items: Vec<RepoSearchResult>,
}

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
    let filters = RepoSearchFilters {
        query: normalize(query.q),
        filter: query.filter,
        language: normalize(query.language),
        license_spdx: normalize(query.license),
        stars_min: query.stars_min.filter(|v| *v >= 0),
        include_archived: query.include_archived,
        limit: query.limit,
    };
    let items = search_github_repos(&state.db, &state.config, &filters).await?;

    Ok(Json(SearchResponse {
        filter: filters.filter,
        items,
    }))
}

fn normalize(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
