use rmcp::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::services::recommendations::UseCaseRecommendation;

use super::common::{Provenance, RiskTolerance};
use super::recommend_explain::{
    recommendation_caveats, recommendation_next_actions, recommendation_reasons,
};
use super::search::{RadarBrief, radar_brief};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RecommendReposParams {
    /// Natural-language need, package category, or dependency use case.
    pub need: String,
    /// Optional ecosystem hint, for example TypeScript, Python, Rust, Go, React.
    #[serde(default)]
    pub language: Option<String>,
    /// Optional ecosystem hint. Prefer this over `language` for product asks such as
    /// React, Node, Python, Rust, Go, Django, or frontend.
    #[serde(default)]
    pub ecosystem: Option<String>,
    /// Risk tolerance: `low` favors strict, boring dependencies; `medium` is balanced;
    /// `high` allows newer or less proven repos when relevance is strong.
    #[serde(default)]
    pub risk_tolerance: Option<String>,
    /// Topics that must appear on the candidate repo when present in the corpus.
    #[serde(default)]
    pub must_have_topics: Vec<String>,
    /// Quality filter preset: auto (default), strict, or explore.
    #[serde(default)]
    pub filter: Option<String>,
    /// Max recommendations to return (default 5, max 10).
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct RepoRecommendation {
    pub rank: usize,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub quality_overall: Option<f64>,
    pub quality_freshness: Option<f64>,
    pub quality_adoption: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub quality_vitality: Option<f64>,
    pub flags: Vec<String>,
    pub radar: Option<RadarBrief>,
    pub reasons: Vec<String>,
    pub caveats: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RecommendReposOutput {
    pub provenance: Provenance,
    pub query_used: String,
    pub ecosystem_used: Option<String>,
    pub risk_tolerance_used: String,
    pub must_have_topics: Vec<String>,
    pub filter_used: String,
    pub count: usize,
    pub recommendations: Vec<RepoRecommendation>,
    pub stable_picks: Vec<RepoRecommendation>,
    pub emerging_picks: Vec<RepoRecommendation>,
    pub fallback_candidates: Vec<String>,
    pub fallback: Option<RecommendationFallback>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct RecommendationSections {
    pub stable_picks: Vec<RepoRecommendation>,
    pub emerging_picks: Vec<RepoRecommendation>,
    pub fallback_candidates: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RecommendationFallback {
    pub message: String,
    pub add_repo_candidates: Vec<String>,
    pub next_actions: Vec<String>,
}

pub(crate) fn build_recommendations(
    results: Vec<crate::domain::repo::RepoSearchResult>,
    risk: RiskTolerance,
) -> Vec<RepoRecommendation> {
    results
        .into_iter()
        .enumerate()
        .map(|(index, repo)| {
            let q = repo.quality.as_ref();
            let radar = repo.radar.as_ref().map(radar_brief);
            RepoRecommendation {
                rank: index + 1,
                owner: repo.owner,
                name: repo.name,
                full_name: repo.full_name,
                html_url: repo.html_url,
                description: repo.description,
                language: repo.language,
                topics: repo.topics,
                stars_count: repo.stars_count,
                quality_overall: q.and_then(|q| q.overall),
                quality_freshness: q.and_then(|q| q.freshness),
                quality_adoption: q.and_then(|q| q.adoption),
                quality_reliability: q.and_then(|q| q.reliability),
                quality_abandonment: q.and_then(|q| q.abandonment),
                quality_vitality: q.and_then(|q| q.vitality),
                flags: q.map(|q| q.flags.clone()).unwrap_or_default(),
                radar,
                reasons: recommendation_reasons(q, repo.radar.as_ref(), risk),
                caveats: recommendation_caveats(q, repo.radar.as_ref(), risk),
                next_actions: recommendation_next_actions(q, repo.radar.as_ref()),
            }
        })
        .collect()
}

pub(crate) fn build_recommendations_from_use_case(
    recommendations: Vec<UseCaseRecommendation>,
    risk: RiskTolerance,
) -> Vec<RepoRecommendation> {
    recommendations
        .into_iter()
        .enumerate()
        .map(|(index, recommendation)| {
            let mut output = build_recommendations(vec![recommendation.repo], risk)
                .into_iter()
                .next()
                .expect("one input repo produces one MCP recommendation");
            output.rank = index + 1;
            output.reasons.insert(0, recommendation.reason);
            output.reasons.push(format!(
                "Use-case match score is {:.3}.",
                recommendation.match_score
            ));
            output.reasons.push(format!(
                "Recommendation score is {:.3}.",
                recommendation.recommendation_score
            ));
            if !recommendation.matched_topics.is_empty() {
                output.reasons.push(format!(
                    "Matched intent topics: {}.",
                    recommendation.matched_topics.join(", ")
                ));
            }
            output
        })
        .collect()
}

pub(crate) fn recommendation_sections(
    recommendations: Vec<RepoRecommendation>,
    fallback_candidates: Vec<String>,
) -> RecommendationSections {
    let mut stable_picks = Vec::new();
    let mut emerging_picks = Vec::new();

    for recommendation in recommendations {
        let band = recommendation
            .radar
            .as_ref()
            .map(|radar| radar.maturity_band.as_str());
        if matches!(band, Some("emerging" | "experimental")) {
            emerging_picks.push(recommendation);
        } else {
            stable_picks.push(recommendation);
        }
    }

    RecommendationSections {
        stable_picks,
        emerging_picks,
        fallback_candidates,
    }
}

#[cfg(test)]
pub(crate) use super::recommend_match::{build_recommendation_query, repo_matches_intent};

#[cfg(test)]
pub(crate) fn build_recommendation_fallback(
    query: &str,
    ecosystem: Option<&str>,
    topics: &[String],
    risk: RiskTolerance,
) -> RecommendationFallback {
    let mut candidate_terms = vec![query.to_string()];
    if let Some(ecosystem) = ecosystem {
        candidate_terms.push(ecosystem.to_string());
    }
    candidate_terms.extend(topics.iter().cloned());
    RecommendationFallback {
        message: "No indexed repo matched the current constraints. Add candidate repos, then retry the recommendation.".to_string(),
        add_repo_candidates: vec![
            format!("Search GitHub for: {}", candidate_terms.join(" ")),
            "Add promising repos with POST /api/repos/add or the UseStakly UI.".to_string(),
            "Retry recommend_github_repos after ingestion and scoring completes.".to_string(),
        ],
        next_actions: vec![
            "Relax must_have_topics if they are too narrow.".to_string(),
            format!(
                "Current risk_tolerance is {}; use high/explore only when relevance matters more than maturity.",
                risk.as_str()
            ),
            "For each candidate, prefer maintained repos with recent commits and clear release activity.".to_string(),
        ],
    }
}
