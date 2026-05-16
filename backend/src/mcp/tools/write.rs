use chrono::{DateTime, Utc};
use rmcp::ErrorData;
use rmcp::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::AppState,
    domain::quality::SignalKind,
    services::{
        ingestion::github::{build_client, ingest_repo},
        repos as repos_service,
    },
};

use super::common::{Provenance, map_api_error};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LogUsageParams {
    pub owner: String,
    pub name: String,
    /// Allowed outcomes: resolve, build_success, build_failure, regret, re_resolve
    pub outcome: String,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct LogUsageOutput {
    pub provenance: Provenance,
    pub owner: String,
    pub name: String,
    pub signal: String,
    pub recorded_at: DateTime<Utc>,
    pub quality_overall: Option<f64>,
    pub quality_adoption: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub quality_resolve_count: i32,
    pub quality_build_success_count: i32,
    pub quality_build_failure_count: i32,
    pub quality_regret_count: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchRepoParams {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct WatchRepoOutput {
    pub provenance: Provenance,
    pub owner: String,
    pub name: String,
    pub artifact_id: String,
    pub watching: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchUseCaseParams {
    /// Natural-language need to monitor, such as `testing tools for TypeScript`.
    pub need: String,
    /// Optional label displayed in UseStakly watchlist.
    #[serde(default)]
    pub label: Option<String>,
    /// Risk tolerance: low, medium, or high.
    #[serde(default)]
    pub risk_tolerance: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct WatchUseCaseOutput {
    pub provenance: Provenance,
    pub watch_id: String,
    pub label: String,
    pub query: String,
    pub normalized_intent: String,
    pub categories: Vec<String>,
    pub topics: Vec<String>,
    pub languages: Vec<String>,
    pub risk_tolerance: String,
    pub enabled: bool,
    pub initial_matches: i64,
    pub top_matches: Vec<WatchUseCaseMatchOutput>,
    pub created_at: DateTime<Utc>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct WatchUseCaseMatchOutput {
    pub artifact_id: String,
    pub full_name: String,
    pub language: Option<String>,
    pub match_score: f64,
    pub quality_score: Option<f64>,
}

pub(crate) fn parse_passive_outcome(input: &str) -> Result<SignalKind, ErrorData> {
    match input.trim().to_ascii_lowercase().as_str() {
        "resolve" => Ok(SignalKind::Resolve),
        "build_success" => Ok(SignalKind::BuildSuccess),
        "build_failure" => Ok(SignalKind::BuildFailure),
        "regret" => Ok(SignalKind::Regret),
        "re_resolve" => Ok(SignalKind::ReResolve),
        _ => Err(ErrorData::invalid_params(
            "outcome must be one of: resolve, build_success, build_failure, regret, re_resolve",
            None,
        )),
    }
}

pub(crate) async fn resolve_artifact_id(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<Option<Uuid>, ErrorData> {
    repos_service::find_github_artifact_id(db, owner, name)
        .await
        .map_err(map_api_error)
}

pub(crate) async fn ensure_github_artifact(
    state: &AppState,
    owner: &str,
    name: &str,
) -> Result<Uuid, ErrorData> {
    if let Some(id) = resolve_artifact_id(&state.db, owner, name).await? {
        return Ok(id);
    }

    let token = state.config.github_token.as_deref().ok_or_else(|| {
        ErrorData::invalid_params(
            format!("repo not ingested: {owner}/{name} and GITHUB_TOKEN is not configured"),
            None,
        )
    })?;

    let client = build_client(token).map_err(map_api_error)?;
    let (id, _, _) = ingest_repo(&client, &state.db, &state.config, owner, name)
        .await
        .map_err(map_api_error)?;
    Ok(id)
}

pub(crate) fn into_watch_use_case_output(
    watch: crate::services::use_case_watches::UseCaseWatch,
    formula_version: String,
    scored_at: Option<DateTime<Utc>>,
) -> WatchUseCaseOutput {
    WatchUseCaseOutput {
        provenance: Provenance {
            source: "usestakly://watch/use-case".to_string(),
            formula_version,
            scored_at,
        },
        watch_id: watch.id.to_string(),
        label: watch.label,
        query: watch.query_text,
        normalized_intent: watch.normalized_intent,
        categories: watch.categories,
        topics: watch.topics,
        languages: watch.languages,
        risk_tolerance: watch.risk_tolerance,
        enabled: watch.enabled,
        initial_matches: watch.match_count,
        top_matches: watch
            .top_matches
            .into_iter()
            .map(|item| WatchUseCaseMatchOutput {
                artifact_id: item.artifact_id.to_string(),
                full_name: item.full_name,
                language: item.language,
                match_score: item.match_score,
                quality_score: item.quality_score,
            })
            .collect(),
        created_at: watch.created_at,
        next_actions: vec![
            "Use the UseStakly watchlist to review this need over time.".to_string(),
            "When a recommended repo becomes a dependency, call watch_repo too.".to_string(),
            "After testing a repo, call log_usage so future recommendations improve.".to_string(),
        ],
    }
}
