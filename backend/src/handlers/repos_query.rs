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
    services::repos::{RepoSearchFilters, RepoSort, get_repo_profile, search_github_repos},
};

#[derive(Debug, Deserialize)]
pub struct RepoSearchQuery {
    pub q: Option<String>,
    #[serde(default)]
    pub filter: SearchFilter,
    pub language: Option<String>,
    pub license: Option<String>,
    pub stars_min: Option<i32>,
    pub topics: Option<String>,
    pub maturity_bands: Option<String>,
    pub maturity_band: Option<String>,
    pub score_min: Option<f64>,
    pub abandonment_max: Option<f64>,
    #[serde(default)]
    pub include_archived: bool,
    pub sort: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSearchResponse {
    pub filter: SearchFilter,
    pub sort: String,
    pub limit: i64,
    pub offset: i64,
    pub count: usize,
    pub has_more: bool,
    pub items: Vec<RepoSearchResult>,
}

pub async fn search_repos(
    State(state): State<AppState>,
    Query(query): Query<RepoSearchQuery>,
) -> Result<Json<RepoSearchResponse>, ApiError> {
    let requested_limit = query.limit.unwrap_or(40).clamp(1, 80);
    let offset = query.offset.unwrap_or_default().max(0);
    let filters = RepoSearchFilters {
        query: normalize(query.q),
        filter: query.filter,
        language: normalize(query.language),
        license_spdx: normalize(query.license),
        stars_min: query.stars_min.filter(|v| *v >= 0),
        topics: parse_topics(query.topics),
        maturity_bands: parse_csv_pair(query.maturity_bands, query.maturity_band),
        score_min: query.score_min.filter(|v| (0.0..=1.0).contains(v)),
        abandonment_max: query.abandonment_max.filter(|v| (0.0..=1.0).contains(v)),
        include_archived: query.include_archived,
        sort: RepoSort::parse(query.sort.as_deref()),
        limit: Some(requested_limit + 1),
        offset: Some(offset),
    };
    let mut items = search_github_repos(&state.db, &state.config, &filters).await?;
    let has_more = items.len() > requested_limit as usize;
    items.truncate(requested_limit as usize);
    Ok(Json(RepoSearchResponse {
        filter: filters.filter,
        sort: filters.sort.as_str().to_string(),
        limit: requested_limit,
        offset,
        count: items.len(),
        has_more,
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
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn parse_topics(value: Option<String>) -> Vec<String> {
    parse_csv(value)
}

fn parse_csv_pair(primary: Option<String>, fallback: Option<String>) -> Vec<String> {
    let mut values = parse_csv(primary);
    for value in parse_csv(fallback) {
        if !values.contains(&value) {
            values.push(value);
        }
    }
    values
}

fn parse_csv(value: Option<String>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(',')
        .map(|topic| topic.trim().to_string())
        .filter(|topic| !topic.is_empty())
        .collect()
}
