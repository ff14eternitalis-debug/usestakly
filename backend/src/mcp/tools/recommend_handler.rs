use http::request::Parts;
use rmcp::ErrorData;

use crate::{
    app::AppState,
    mcp::{
        auth::verify_bearer,
        tools::{
            Provenance, RecommendReposOutput, RecommendReposParams, RecommendationFallback,
            map_api_error,
        },
    },
    services::{quality::load_v2, recommendations::recommend_for_use_case},
};

use super::{
    build_recommendations_from_use_case, build_use_case_service_query, map_anyhow,
    normalize_topics, parse_risk_tolerance, recommendation_sections, repo_matches_topics,
};

pub async fn handle_recommend_github_repos(
    state: &AppState,
    p: RecommendReposParams,
    parts: Parts,
) -> Result<RecommendReposOutput, ErrorData> {
    verify_bearer(&state.db, &parts).await?;

    let query = p.need.trim();
    if query.is_empty() {
        return Err(ErrorData::invalid_params("need is required", None));
    }

    let ecosystem = p.ecosystem.as_deref().or(p.language.as_deref());
    let normalized_topics = normalize_topics(&p.must_have_topics);
    let risk_tolerance = parse_risk_tolerance(p.risk_tolerance.as_deref());
    let service_query = build_use_case_service_query(query, ecosystem, &normalized_topics);
    let mut report = recommend_for_use_case(
        &state.db,
        &state.config,
        &service_query,
        risk_tolerance.as_str(),
        (p.limit.unwrap_or(5).clamp(1, 10) * 4).clamp(10, 40),
    )
    .await
    .map_err(map_api_error)?;
    if !normalized_topics.is_empty() {
        report
            .recommendations
            .retain(|recommendation| repo_matches_topics(&recommendation.repo, &normalized_topics));
    }
    let max_results = p.limit.unwrap_or(5).clamp(1, 10) as usize;
    report.recommendations.truncate(max_results);
    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    let scored_at = report
        .recommendations
        .iter()
        .filter_map(|recommendation| {
            recommendation
                .repo
                .quality
                .as_ref()
                .map(|quality| quality.computed_at)
        })
        .max();
    let recommendations =
        build_recommendations_from_use_case(report.recommendations, risk_tolerance);
    let fallback = if recommendations.is_empty() {
        Some(RecommendationFallback {
            message: "No indexed repo matched the current need. Add candidate repos, then retry the recommendation.".to_string(),
            add_repo_candidates: report.fallback_candidates.clone(),
            next_actions: vec![
                "Add promising fallback repos through /discover or POST /api/repos/add.".to_string(),
                "Retry recommend_github_repos after ingestion and scoring completes.".to_string(),
                "Relax must_have_topics if they are too narrow.".to_string(),
            ],
        })
    } else {
        None
    };
    let sections = recommendation_sections(
        recommendations.clone(),
        fallback
            .as_ref()
            .map(|fallback| fallback.add_repo_candidates.clone())
            .unwrap_or_default(),
    );

    Ok(RecommendReposOutput {
        provenance: Provenance {
            source: "usestakly://registry/github/recommendations".to_string(),
            formula_version,
            scored_at,
        },
        query_used: report.query,
        ecosystem_used: ecosystem.map(str::to_string),
        risk_tolerance_used: risk_tolerance.as_str().to_string(),
        must_have_topics: normalized_topics,
        filter_used: p.filter.as_deref().unwrap_or("use_case").to_string(),
        count: recommendations.len(),
        recommendations,
        stable_picks: sections.stable_picks,
        emerging_picks: sections.emerging_picks,
        fallback_candidates: sections.fallback_candidates,
        fallback,
    })
}
