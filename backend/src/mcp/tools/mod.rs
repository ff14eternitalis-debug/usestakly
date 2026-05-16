mod common;
mod context;
mod recommend;
mod recommend_explain;
mod recommend_match;
mod search;
mod write;

pub use common::Provenance;
pub use context::{RepoContextOutput, RepoContextParams, SignalSummary, VitalityInputsOutput};
pub use recommend::{
    RecommendReposOutput, RecommendReposParams, RecommendationFallback, RecommendationSections,
    RepoRecommendation,
};
pub use search::{RadarBrief, RepoCandidate, SearchReposOutput, SearchReposParams};
pub use write::{
    LogUsageOutput, LogUsageParams, WatchRepoOutput, WatchRepoParams, WatchUseCaseMatchOutput,
    WatchUseCaseOutput, WatchUseCaseParams,
};

pub(crate) use common::{
    map_anyhow, map_api_error, mcp_allowed_hosts, normalize_topics, parse_filter,
    parse_risk_tolerance,
};
pub(crate) use context::into_context_output;
pub(crate) use recommend::{build_recommendations_from_use_case, recommendation_sections};
pub(crate) use recommend_match::{build_use_case_service_query, repo_matches_topics};
pub(crate) use search::into_repo_candidate;
pub(crate) use write::{
    ensure_github_artifact, into_watch_use_case_output, parse_passive_outcome, resolve_artifact_id,
};

#[cfg(test)]
pub(crate) use common::RiskTolerance;
#[cfg(test)]
pub(crate) use recommend::{
    build_recommendation_fallback, build_recommendation_query, build_recommendations,
    repo_matches_intent,
};
