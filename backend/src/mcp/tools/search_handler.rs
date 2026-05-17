use http::request::Parts;
use rmcp::ErrorData;

use crate::{
    app::AppState,
    mcp::{
        auth::verify_bearer,
        tools::{
            Provenance, RepoCandidate, SearchReposOutput, SearchReposParams, into_repo_candidate,
            map_api_error,
        },
    },
    services::{
        quality::load_v2,
        repos::{self as repos_service, RepoSearchFilters, RepoSort},
    },
};

use super::{map_anyhow, parse_filter};

pub async fn handle_search_github_repos(
    state: &AppState,
    p: SearchReposParams,
    parts: Parts,
) -> Result<SearchReposOutput, ErrorData> {
    verify_bearer(&state.db, &parts).await?;

    let filter = parse_filter(p.filter.as_deref());
    let sort = RepoSort::parse(p.sort.as_deref());
    let filters = RepoSearchFilters {
        query: p.query,
        filter,
        language: p.language,
        license_spdx: None,
        stars_min: p.stars_min,
        topics: Vec::new(),
        maturity_bands: p.maturity_bands,
        score_min: None,
        abandonment_max: None,
        include_archived: false,
        sort,
        limit: Some(p.limit.unwrap_or(20).clamp(1, 50)),
        offset: None,
    };

    let results = repos_service::search_github_repos(&state.db, &state.config, &filters)
        .await
        .map_err(map_api_error)?;

    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    let scored_at = results
        .iter()
        .filter_map(|r| r.quality.as_ref().map(|q| q.computed_at))
        .max();

    let candidates: Vec<RepoCandidate> = results.into_iter().map(into_repo_candidate).collect();

    Ok(SearchReposOutput {
        provenance: Provenance {
            source: "usestakly://registry/github".to_string(),
            formula_version,
            scored_at,
        },
        filter_used: filter.as_str().to_string(),
        sort_used: sort.as_str().to_string(),
        count: candidates.len(),
        results: candidates,
    })
}
