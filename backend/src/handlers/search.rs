use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::reference::{SearchFilter, SearchResult},
    services::search::search_snippets,
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    #[serde(default)]
    pub filter: SearchFilter,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub filter: SearchFilter,
    pub items: Vec<SearchResult>,
}

pub async fn search(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
    let user_id = resolve_current_user(&state.db, &state.config, &headers)
        .await
        .ok()
        .map(|u| u.id);

    let trimmed = query.q.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let items = search_snippets(&state.db, trimmed, query.filter, user_id).await?;

    Ok(Json(SearchResponse {
        filter: query.filter,
        items,
    }))
}
