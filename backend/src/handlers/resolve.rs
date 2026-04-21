use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
};
use serde::Deserialize;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::reference::{ResolvedSnippet, parse_reference},
    services::resolution::resolve_reference,
};

#[derive(Debug, Deserialize)]
pub struct ResolveQuery {
    #[serde(rename = "ref")]
    pub reference: String,
}

pub async fn resolve(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<ResolveQuery>,
) -> Result<Json<ResolvedSnippet>, ApiError> {
    let parsed = parse_reference(&query.reference)?;
    let user_id = resolve_current_user(&state.db, &state.config, &headers)
        .await
        .ok()
        .map(|u| u.id);
    let resolved = resolve_reference(&state.db, &parsed, user_id).await?;
    Ok(Json(resolved))
}
