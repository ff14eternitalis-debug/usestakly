use std::sync::Arc;

use chrono::{DateTime, Utc};
use http::{Uri, request::Parts};
use rmcp::{
    ErrorData, ServerHandler,
    handler::server::{
        tool::Extension,
        wrapper::{Json, Parameters},
    },
    schemars, tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, session::local::LocalSessionManager,
        tower::StreamableHttpService,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::AppState,
    domain::{
        quality::{ArtifactKind, SignalKind},
        reference::SearchFilter,
    },
    mcp::auth::{verify_agent, verify_bearer},
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::{RecordSignalInput, load_v1, recompute_all_scores_with_config, record_signal},
        repos::{self as repos_service, RepoSearchFilters},
        trust::agent_token_events,
        watchlist,
    },
};

// ---------- MCP tool I/O types ----------
//
// Kept separate from domain types so MCP stays a serialization boundary:
// JsonSchema generation doesn't leak into business code.

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Provenance {
    pub source: String,
    pub formula_version: String,
    pub scored_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchReposParams {
    /// Lexical query. Matched against owner, repo name, description, and topics.
    #[serde(default)]
    pub query: Option<String>,
    /// Quality filter preset: `auto` (default) excludes unreliable/abandoned repos,
    /// `strict` keeps only the most trusted, `explore` disables quality gates.
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub stars_min: Option<i32>,
    /// Max number of results (default 20, max 50).
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RepoCandidate {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub quality_overall: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub flags: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SearchReposOutput {
    pub provenance: Provenance,
    pub filter_used: String,
    pub count: usize,
    pub results: Vec<RepoCandidate>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RecommendReposParams {
    /// Natural-language need, package category, or dependency use case.
    pub need: String,
    /// Optional ecosystem hint, for example TypeScript, Python, Rust, Go, React.
    #[serde(default)]
    pub language: Option<String>,
    /// Quality filter preset: auto (default), strict, or explore.
    #[serde(default)]
    pub filter: Option<String>,
    /// Max recommendations to return (default 5, max 10).
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RepoRecommendation {
    pub rank: usize,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars_count: i32,
    pub quality_overall: Option<f64>,
    pub quality_freshness: Option<f64>,
    pub quality_adoption: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub flags: Vec<String>,
    pub reasons: Vec<String>,
    pub caveats: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RecommendReposOutput {
    pub provenance: Provenance,
    pub query_used: String,
    pub filter_used: String,
    pub count: usize,
    pub recommendations: Vec<RepoRecommendation>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RepoContextParams {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RepoContextOutput {
    pub provenance: Provenance,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub default_branch: Option<String>,
    pub quality_overall: Option<f64>,
    pub quality_freshness: Option<f64>,
    pub quality_adoption: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub quality_resolve_count: i32,
    pub quality_build_success_count: i32,
    pub quality_build_failure_count: i32,
    pub quality_regret_count: i32,
    pub flags: Vec<String>,
    pub recent_signals: Vec<SignalSummary>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SignalSummary {
    pub signal: String,
    pub is_passive: bool,
    pub evidence_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

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

// ---------- Server handler ----------

#[derive(Clone)]
pub struct McpServer {
    state: AppState,
}

impl McpServer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tool_router]
impl McpServer {
    #[tool(
        name = "search_github_repos",
        description = "Search scored GitHub repos in UseStakly's public registry. \
                       Returns candidates ranked by quality (`overall` score) and stars. \
                       Use `filter='auto'` (default) for a safe shortlist, \
                       `filter='strict'` for the most trusted repos only, \
                       `filter='explore'` to bypass quality gates."
    )]
    async fn search_github_repos(
        &self,
        Parameters(p): Parameters<SearchReposParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<SearchReposOutput>, ErrorData> {
        verify_bearer(&self.state.db, &parts).await?;

        let filter = parse_filter(p.filter.as_deref());
        let filters = RepoSearchFilters {
            query: p.query,
            filter,
            language: p.language,
            license_spdx: None,
            stars_min: p.stars_min,
            include_archived: false,
            limit: Some(p.limit.unwrap_or(20).clamp(1, 50)),
        };

        let results =
            repos_service::search_github_repos(&self.state.db, &self.state.config, &filters)
                .await
                .map_err(map_api_error)?;

        let formula_version = load_v1().map_err(map_anyhow)?.meta.version;
        let scored_at = results
            .iter()
            .filter_map(|r| r.quality.as_ref().map(|q| q.computed_at))
            .max();

        let candidates: Vec<RepoCandidate> = results
            .into_iter()
            .map(|r| RepoCandidate {
                owner: r.owner,
                name: r.name,
                full_name: r.full_name,
                html_url: r.html_url,
                description: r.description,
                language: r.language,
                license_spdx: r.license_spdx,
                topics: r.topics,
                stars_count: r.stars_count,
                archived: r.archived,
                last_commit_at: r.last_commit_at,
                quality_overall: r.quality.as_ref().and_then(|q| q.overall),
                quality_reliability: r.quality.as_ref().and_then(|q| q.reliability),
                quality_abandonment: r.quality.as_ref().and_then(|q| q.abandonment),
                flags: r.quality.map(|q| q.flags).unwrap_or_default(),
            })
            .collect();

        Ok(Json(SearchReposOutput {
            provenance: Provenance {
                source: "usestakly://registry/github".to_string(),
                formula_version,
                scored_at,
            },
            filter_used: filter.as_str().to_string(),
            count: candidates.len(),
            results: candidates,
        }))
    }

    #[tool(
        name = "recommend_github_repos",
        description = "Recommend a short, explained list of GitHub repositories for a dependency \
                       need, such as `I need a reliable TypeScript ORM` or `React table library`. \
                       Returns ranked candidates with score-based reasons, caveats, next actions, \
                       and provenance. Use this before choosing a dependency."
    )]
    async fn recommend_github_repos(
        &self,
        Parameters(p): Parameters<RecommendReposParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<RecommendReposOutput>, ErrorData> {
        verify_bearer(&self.state.db, &parts).await?;

        let query = p.need.trim();
        if query.is_empty() {
            return Err(ErrorData::invalid_params("need is required", None));
        }

        let filter = parse_filter(p.filter.as_deref());
        let filters = RepoSearchFilters {
            query: Some(query.to_string()),
            filter,
            language: p.language,
            license_spdx: None,
            stars_min: None,
            include_archived: false,
            limit: Some(p.limit.unwrap_or(5).clamp(1, 10)),
        };

        let results =
            repos_service::search_github_repos(&self.state.db, &self.state.config, &filters)
                .await
                .map_err(map_api_error)?;
        let formula_version = load_v1().map_err(map_anyhow)?.meta.version;
        let scored_at = results
            .iter()
            .filter_map(|r| r.quality.as_ref().map(|q| q.computed_at))
            .max();
        let recommendations = build_recommendations(results);

        Ok(Json(RecommendReposOutput {
            provenance: Provenance {
                source: "usestakly://registry/github/recommendations".to_string(),
                formula_version,
                scored_at,
            },
            query_used: query.to_string(),
            filter_used: filter.as_str().to_string(),
            count: recommendations.len(),
            recommendations,
        }))
    }

    #[tool(
        name = "get_repo_quality_context",
        description = "Fetch a full quality profile for one GitHub repo: priors, \
                       multi-dimensional score (freshness/adoption/reliability/abandonment), \
                       active flags, and up to 10 recent signals. Use after \
                       `search_github_repos` to justify the pick."
    )]
    async fn get_repo_quality_context(
        &self,
        Parameters(p): Parameters<RepoContextParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<RepoContextOutput>, ErrorData> {
        verify_bearer(&self.state.db, &parts).await?;

        let owner = p.owner.trim();
        let name = p.name.trim();
        if owner.is_empty() || name.is_empty() {
            return Err(ErrorData::invalid_params(
                "owner and name are required",
                None,
            ));
        }

        let artifact_id = resolve_artifact_id(&self.state.db, owner, name)
            .await?
            .ok_or_else(|| {
                ErrorData::invalid_params(format!("repo not ingested: {owner}/{name}"), None)
            })?;

        let profile = repos_service::get_repo_profile(&self.state.db, artifact_id)
            .await
            .map_err(map_api_error)?;

        let formula_version = load_v1().map_err(map_anyhow)?.meta.version;
        Ok(Json(into_context_output(profile, formula_version)))
    }

    #[tool(
        name = "log_usage",
        description = "Record passive usage feedback for one GitHub repo after the agent tried it. \
                       Allowed outcomes are passive-only: resolve, build_success, build_failure, \
                       regret, re_resolve."
    )]
    async fn log_usage(
        &self,
        Parameters(p): Parameters<LogUsageParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<LogUsageOutput>, ErrorData> {
        let agent = verify_agent(&self.state.db, &parts).await?;
        let owner = p.owner.trim();
        let name = p.name.trim();
        if owner.is_empty() || name.is_empty() {
            return Err(ErrorData::invalid_params(
                "owner and name are required",
                None,
            ));
        }

        let signal = parse_passive_outcome(&p.outcome)?;
        let notes = p.notes.as_deref().map(str::trim).filter(|s| !s.is_empty());
        agent_token_events::enforce_write_quota(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            agent_token_events::REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            self.state.config.mcp_write_limit_per_hour,
        )
        .await
        .map_err(map_api_error)?;
        agent_token_events::enforce_log_usage_guards(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            owner,
            name,
            signal,
            notes,
            self.state.config.mcp_log_usage_cooldown_secs,
            self.state.config.mcp_negative_signal_window_hours,
        )
        .await
        .map_err(map_api_error)?;
        let artifact_id = ensure_github_artifact(&self.state, owner, name).await?;

        let record = record_signal(
            &self.state.db,
            RecordSignalInput {
                artifact_kind: ArtifactKind::External,
                snippet_id: None,
                external_artifact_id: Some(artifact_id),
                signal,
                review_status: "accepted".to_string(),
                actor_user_id: Some(agent.user_id),
                evidence_url: None,
                evidence_description: None,
                agent_context: Some(json!({
                    "source": "mcp",
                    "token_id": agent.token_id,
                    "notes": notes,
                })),
            },
        )
        .await
        .map_err(map_api_error)?;
        agent_token_events::record_log_usage(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            owner,
            name,
            signal,
            notes,
        )
        .await
        .map_err(map_api_error)?;
        let report = recompute_all_scores_with_config(&self.state.db, Some(&self.state.config))
            .await
            .map_err(map_anyhow)?;

        let formula_version = load_v1().map_err(map_anyhow)?.meta.version;
        let profile = repos_service::get_repo_profile(&self.state.db, artifact_id)
            .await
            .map_err(map_api_error)?;
        let q = profile.repo.quality.as_ref();
        Ok(Json(LogUsageOutput {
            provenance: Provenance {
                source: format!("usestakly://registry/github/{owner}/{name}"),
                formula_version,
                scored_at: Some(report.computed_at),
            },
            owner: owner.to_string(),
            name: name.to_string(),
            signal: record.signal,
            recorded_at: record.created_at,
            quality_overall: q.and_then(|q| q.overall),
            quality_adoption: q.and_then(|q| q.adoption),
            quality_reliability: q.and_then(|q| q.reliability),
            quality_abandonment: q.and_then(|q| q.abandonment),
            quality_resolve_count: q.map(|q| q.resolve_count).unwrap_or_default(),
            quality_build_success_count: q.map(|q| q.build_success_count).unwrap_or_default(),
            quality_build_failure_count: q.map(|q| q.build_failure_count).unwrap_or_default(),
            quality_regret_count: q.map(|q| q.regret_count).unwrap_or_default(),
        }))
    }

    #[tool(
        name = "watch_repo",
        description = "Add one GitHub repo to the authenticated user's watchlist so UseStakly can \
                       notify them when quality drops, abandonment rises, or severe flags appear."
    )]
    async fn watch_repo(
        &self,
        Parameters(p): Parameters<WatchRepoParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<WatchRepoOutput>, ErrorData> {
        let agent = verify_agent(&self.state.db, &parts).await?;
        let owner = p.owner.trim();
        let name = p.name.trim();
        if owner.is_empty() || name.is_empty() {
            return Err(ErrorData::invalid_params(
                "owner and name are required",
                None,
            ));
        }

        agent_token_events::enforce_write_quota(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            agent_token_events::REJECTION_TOOL_WATCH_REPO,
            owner,
            name,
            self.state.config.mcp_write_limit_per_hour,
        )
        .await
        .map_err(map_api_error)?;
        let artifact_id = ensure_github_artifact(&self.state, owner, name).await?;
        watchlist::add_watch(&self.state.db, agent.user_id, artifact_id)
            .await
            .map_err(map_api_error)?;
        agent_token_events::record_watch_repo(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            owner,
            name,
        )
        .await
        .map_err(map_api_error)?;

        let formula_version = load_v1().map_err(map_anyhow)?.meta.version;
        Ok(Json(WatchRepoOutput {
            provenance: Provenance {
                source: format!("usestakly://registry/github/{owner}/{name}"),
                formula_version,
                scored_at: None,
            },
            owner: owner.to_string(),
            name: name.to_string(),
            artifact_id: artifact_id.to_string(),
            watching: true,
        }))
    }
}

#[tool_handler(
    name = "usestakly-mcp",
    instructions = "UseStakly MCP — query a scored registry of public GitHub repos. \
                    Always call `search_github_repos` before generating code that pulls in \
                    a dependency, then `get_repo_quality_context` to confirm the pick. \
                    After trying a repo, call `log_usage`. Use `watch_repo` when the user wants \
                    ongoing monitoring. Write calls are rate-limited per token and duplicate \
                    `log_usage` events are intentionally throttled. Include the returned provenance \
                    string when you write the code."
)]
impl ServerHandler for McpServer {}

// ---------- Helpers ----------

fn parse_filter(s: Option<&str>) -> SearchFilter {
    match s.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        Some("strict") => SearchFilter::Strict,
        Some("explore") => SearchFilter::Explore,
        _ => SearchFilter::Auto,
    }
}

fn map_api_error(e: crate::app::error::ApiError) -> ErrorData {
    ErrorData::internal_error(format!("service error: {}", e.message), None)
}

fn map_anyhow(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(format!("scoring error: {e}"), None)
}

async fn resolve_artifact_id(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<Option<Uuid>, ErrorData> {
    repos_service::find_github_artifact_id(db, owner, name)
        .await
        .map_err(map_api_error)
}

async fn ensure_github_artifact(
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
    let (id, _) = ingest_repo(&client, &state.db, &state.config, owner, name)
        .await
        .map_err(map_api_error)?;
    Ok(id)
}

fn parse_passive_outcome(input: &str) -> Result<SignalKind, ErrorData> {
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

fn build_recommendations(
    results: Vec<crate::domain::repo::RepoSearchResult>,
) -> Vec<RepoRecommendation> {
    results
        .into_iter()
        .enumerate()
        .map(|(index, repo)| {
            let q = repo.quality.as_ref();
            RepoRecommendation {
                rank: index + 1,
                owner: repo.owner,
                name: repo.name,
                full_name: repo.full_name,
                html_url: repo.html_url,
                description: repo.description,
                language: repo.language,
                stars_count: repo.stars_count,
                quality_overall: q.and_then(|q| q.overall),
                quality_freshness: q.and_then(|q| q.freshness),
                quality_adoption: q.and_then(|q| q.adoption),
                quality_reliability: q.and_then(|q| q.reliability),
                quality_abandonment: q.and_then(|q| q.abandonment),
                flags: q.map(|q| q.flags.clone()).unwrap_or_default(),
                reasons: recommendation_reasons(q),
                caveats: recommendation_caveats(q),
                next_actions: vec![
                    "Call get_repo_quality_context before final selection.".to_string(),
                    "After testing the dependency, call log_usage with the outcome.".to_string(),
                    "Use watch_repo if this becomes a dependency to monitor.".to_string(),
                ],
            }
        })
        .collect()
}

fn recommendation_reasons(
    quality: Option<&crate::domain::reference::QualityContext>,
) -> Vec<String> {
    let Some(q) = quality else {
        return vec!["No score is available yet; inspect the repo before adopting.".to_string()];
    };
    let mut reasons = Vec::new();
    if let Some(overall) = q.overall {
        reasons.push(format!("Overall dependency score is {:.3}.", overall));
    }
    if q.freshness.unwrap_or(0.0) >= 0.8 {
        reasons.push("Freshness is strong, indicating recent repository activity.".to_string());
    }
    if q.abandonment.unwrap_or(1.0) <= 0.2 {
        reasons.push("Abandonment risk is currently low.".to_string());
    }
    if q.reliability.unwrap_or(0.5) > 0.5 {
        reasons.push("Reliability is supported by positive usage outcomes.".to_string());
    } else if q.build_success_count > 0 || q.build_failure_count > 0 {
        reasons.push(format!(
            "Reliability has {} build success and {} build failure signals.",
            q.build_success_count, q.build_failure_count
        ));
    }
    if reasons.is_empty() {
        reasons.push(
            "Included because it matched the query and passed the selected filter.".to_string(),
        );
    }
    reasons
}

fn recommendation_caveats(
    quality: Option<&crate::domain::reference::QualityContext>,
) -> Vec<String> {
    let Some(q) = quality else {
        return vec!["Score provenance is missing until the repo is computed.".to_string()];
    };
    let mut caveats = Vec::new();
    if q.reliability.unwrap_or(0.5) == 0.5 && q.build_success_count + q.build_failure_count < 5 {
        caveats.push(
            "Reliability is still neutral because there are fewer than 5 build samples."
                .to_string(),
        );
    }
    if q.adoption.unwrap_or(0.0) == 0.0 && q.resolve_count == 0 {
        caveats.push(
            "Adoption has no usage outcomes yet; treat popularity separately from proven usage."
                .to_string(),
        );
    }
    if !q.flags.is_empty() {
        caveats.push(format!("Active flags to inspect: {}.", q.flags.join(", ")));
    }
    if q.abandonment.unwrap_or(0.0) > 0.4 {
        caveats
            .push("Abandonment risk is elevated; inspect maintenance before adoption.".to_string());
    }
    caveats
}

fn into_context_output(
    profile: crate::domain::repo::RepoProfile,
    formula_version: String,
) -> RepoContextOutput {
    let q = profile.repo.quality.clone();
    let scored_at = q.as_ref().map(|q| q.computed_at);
    RepoContextOutput {
        provenance: Provenance {
            source: format!(
                "usestakly://registry/github/{}/{}",
                profile.repo.owner, profile.repo.name
            ),
            formula_version,
            scored_at,
        },
        owner: profile.repo.owner,
        name: profile.repo.name,
        full_name: profile.repo.full_name,
        html_url: profile.repo.html_url,
        description: profile.repo.description,
        language: profile.repo.language,
        license_spdx: profile.repo.license_spdx,
        topics: profile.repo.topics,
        stars_count: profile.repo.stars_count,
        forks_count: profile.repo.forks_count,
        open_issues_count: profile.repo.open_issues_count,
        subscribers_count: profile.subscribers_count,
        archived: profile.repo.archived,
        last_commit_at: profile.repo.last_commit_at,
        default_branch: profile.default_branch,
        quality_overall: q.as_ref().and_then(|q| q.overall),
        quality_freshness: q.as_ref().and_then(|q| q.freshness),
        quality_adoption: q.as_ref().and_then(|q| q.adoption),
        quality_reliability: q.as_ref().and_then(|q| q.reliability),
        quality_abandonment: q.as_ref().and_then(|q| q.abandonment),
        quality_resolve_count: q.as_ref().map(|q| q.resolve_count).unwrap_or_default(),
        quality_build_success_count: q
            .as_ref()
            .map(|q| q.build_success_count)
            .unwrap_or_default(),
        quality_build_failure_count: q
            .as_ref()
            .map(|q| q.build_failure_count)
            .unwrap_or_default(),
        quality_regret_count: q.as_ref().map(|q| q.regret_count).unwrap_or_default(),
        flags: q.map(|q| q.flags).unwrap_or_default(),
        recent_signals: profile
            .recent_signals
            .into_iter()
            .map(|s| SignalSummary {
                signal: s.signal,
                is_passive: s.is_passive,
                evidence_url: s.evidence_url,
                created_at: s.created_at,
            })
            .collect(),
    }
}

// ---------- Axum integration ----------

/// Build the tower service mounted by `app::build_app` at `/mcp`.
pub fn build_service(state: AppState) -> StreamableHttpService<McpServer, LocalSessionManager> {
    let config =
        StreamableHttpServerConfig::default().with_allowed_hosts(mcp_allowed_hosts(&state.config));
    StreamableHttpService::new(
        move || Ok(McpServer::new(state.clone())),
        Arc::new(LocalSessionManager::default()),
        config,
    )
}

fn mcp_allowed_hosts(config: &crate::config::AppConfig) -> Vec<String> {
    let mut hosts = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ];

    for value in [
        config.app_base_url.as_str(),
        config.frontend_base_url.as_str(),
    ] {
        if let Ok(uri) = value.parse::<Uri>()
            && let Some(authority) = uri.authority()
        {
            push_unique(&mut hosts, authority.as_str().to_string());
            push_unique(&mut hosts, authority.host().to_string());
        }
    }

    push_unique(&mut hosts, config.host.clone());
    hosts
}

fn push_unique(values: &mut Vec<String>, value: String) {
    let value = value.trim().trim_matches(['[', ']']).to_string();
    if value.is_empty() || values.iter().any(|existing| existing == &value) {
        return;
    }
    values.push(value);
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    use super::*;
    use crate::config::AppConfig;
    use crate::domain::{
        reference::{QualityContext, SearchFilter},
        repo::{RepoProfile, RepoSearchResult, RepoSignal},
    };

    #[test]
    fn parse_filter_defaults_to_auto_for_missing_or_unknown_values() {
        assert_eq!(parse_filter(None), SearchFilter::Auto);
        assert_eq!(parse_filter(Some("")), SearchFilter::Auto);
        assert_eq!(parse_filter(Some("anything-else")), SearchFilter::Auto);
    }

    #[test]
    fn parse_filter_accepts_case_insensitive_agent_values() {
        assert_eq!(parse_filter(Some(" STRICT ")), SearchFilter::Strict);
        assert_eq!(parse_filter(Some("explore")), SearchFilter::Explore);
        assert_eq!(parse_filter(Some("Auto")), SearchFilter::Auto);
    }

    #[test]
    fn parse_passive_outcome_accepts_only_mcp_write_outcomes() {
        assert_eq!(
            parse_passive_outcome("resolve").unwrap(),
            SignalKind::Resolve
        );
        assert_eq!(
            parse_passive_outcome(" BUILD_SUCCESS ").unwrap(),
            SignalKind::BuildSuccess
        );
        assert_eq!(
            parse_passive_outcome("build_failure").unwrap(),
            SignalKind::BuildFailure
        );
        assert_eq!(parse_passive_outcome("regret").unwrap(), SignalKind::Regret);
        assert_eq!(
            parse_passive_outcome("re_resolve").unwrap(),
            SignalKind::ReResolve
        );

        assert!(parse_passive_outcome("security_issue").is_err());
        assert!(parse_passive_outcome("deprecated").is_err());
        assert!(parse_passive_outcome("broken").is_err());
    }

    #[test]
    fn mcp_allowed_hosts_include_public_backend_authority() {
        let config = test_config(
            "https://xl4xtxfxbxm0lvqjywsl98il.137.74.112.197.sslip.io",
            "https://gii8ev9tkvjffvoqdiyxp6p3.137.74.112.197.sslip.io",
        );

        let hosts = mcp_allowed_hosts(&config);

        assert!(hosts.contains(&"localhost".to_string()));
        assert!(hosts.contains(&"127.0.0.1".to_string()));
        assert!(hosts.contains(&"xl4xtxfxbxm0lvqjywsl98il.137.74.112.197.sslip.io".to_string()));
        assert!(hosts.contains(&"gii8ev9tkvjffvoqdiyxp6p3.137.74.112.197.sslip.io".to_string()));
    }

    #[test]
    fn context_output_preserves_provenance_quality_and_recent_signals() {
        let computed_at = Utc.with_ymd_and_hms(2026, 4, 24, 8, 0, 0).unwrap();
        let signal_at = Utc.with_ymd_and_hms(2026, 4, 24, 9, 0, 0).unwrap();
        let artifact_id = Uuid::parse_str("11111111-1111-4111-8111-111111111111").unwrap();
        let signal_id = Uuid::parse_str("22222222-2222-4222-8222-222222222222").unwrap();

        let profile = RepoProfile {
            repo: RepoSearchResult {
                artifact_id,
                owner: "facebook".to_string(),
                name: "react".to_string(),
                full_name: "facebook/react".to_string(),
                html_url: "https://github.com/facebook/react".to_string(),
                description: Some("The library for web and native user interfaces.".to_string()),
                language: Some("JavaScript".to_string()),
                license_spdx: Some("MIT".to_string()),
                topics: vec!["ui".to_string(), "react".to_string()],
                stars_count: 235_000,
                forks_count: 48_000,
                open_issues_count: 1_200,
                archived: false,
                last_commit_at: Some(computed_at),
                quality: Some(QualityContext {
                    formula_version: "v1.1".to_string(),
                    freshness: Some(0.91),
                    adoption: Some(0.98),
                    reliability: Some(0.84),
                    abandonment: Some(0.08),
                    overall: Some(0.9),
                    resolve_count: 12,
                    build_success_count: 8,
                    build_failure_count: 1,
                    regret_count: 0,
                    flags: vec!["deprecated".to_string()],
                    computed_at,
                }),
            },
            subscribers_count: 6_400,
            default_branch: Some("main".to_string()),
            priors_fetched_at: Some(computed_at),
            recent_signals: vec![RepoSignal {
                id: signal_id,
                signal: "build_success".to_string(),
                is_passive: true,
                evidence_url: None,
                evidence_description: Some("Smoke test passed.".to_string()),
                review_status: "accepted".to_string(),
                review_note: None,
                disputed_at: None,
                dispute_reason: None,
                created_at: signal_at,
                events: Vec::new(),
            }],
        };

        let output = into_context_output(profile, "v1.1".to_string());

        assert_eq!(
            output.provenance.source,
            "usestakly://registry/github/facebook/react"
        );
        assert_eq!(output.provenance.formula_version, "v1.1");
        assert_eq!(output.provenance.scored_at, Some(computed_at));
        assert_eq!(output.full_name, "facebook/react");
        assert_eq!(output.quality_overall, Some(0.9));
        assert_eq!(output.quality_abandonment, Some(0.08));
        assert_eq!(output.quality_resolve_count, 12);
        assert_eq!(output.quality_build_success_count, 8);
        assert_eq!(output.quality_build_failure_count, 1);
        assert_eq!(output.quality_regret_count, 0);
        assert_eq!(output.flags, vec!["deprecated"]);
        assert_eq!(output.recent_signals.len(), 1);
        assert_eq!(output.recent_signals[0].signal, "build_success");
        assert!(output.recent_signals[0].is_passive);
    }

    #[test]
    fn recommendations_explain_score_caveats_and_next_actions() {
        let computed_at = Utc.with_ymd_and_hms(2026, 4, 25, 10, 0, 0).unwrap();
        let artifact_id = Uuid::parse_str("33333333-3333-4333-8333-333333333333").unwrap();
        let results = vec![RepoSearchResult {
            artifact_id,
            owner: "example".to_string(),
            name: "typed-orm".to_string(),
            full_name: "example/typed-orm".to_string(),
            html_url: "https://github.com/example/typed-orm".to_string(),
            description: Some("A TypeScript ORM.".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: Some("MIT".to_string()),
            topics: vec!["orm".to_string(), "typescript".to_string()],
            stars_count: 12_000,
            forks_count: 600,
            open_issues_count: 30,
            archived: false,
            last_commit_at: Some(computed_at),
            quality: Some(QualityContext {
                formula_version: "v1.1".to_string(),
                freshness: Some(0.92),
                adoption: Some(0.0),
                reliability: Some(0.5),
                abandonment: Some(0.04),
                overall: Some(0.71),
                resolve_count: 0,
                build_success_count: 1,
                build_failure_count: 0,
                regret_count: 0,
                flags: Vec::new(),
                computed_at,
            }),
        }];

        let recommendations = build_recommendations(results);

        assert_eq!(recommendations.len(), 1);
        assert_eq!(recommendations[0].rank, 1);
        assert_eq!(recommendations[0].full_name, "example/typed-orm");
        assert!(
            recommendations[0]
                .reasons
                .iter()
                .any(|reason| reason.contains("Overall dependency score"))
        );
        assert!(
            recommendations[0]
                .caveats
                .iter()
                .any(|caveat| caveat.contains("Reliability is still neutral"))
        );
        assert!(
            recommendations[0]
                .next_actions
                .iter()
                .any(|action| action.contains("get_repo_quality_context"))
        );
    }

    fn test_config(app_base_url: &str, frontend_base_url: &str) -> AppConfig {
        AppConfig {
            host: "127.0.0.1".to_string(),
            port: 4000,
            database_url: "postgres://example".to_string(),
            dev_user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            dev_user_email: "dev@usestakly.local".to_string(),
            dev_user_username: "dev".to_string(),
            dev_user_display_name: None,
            dev_user_avatar_url: None,
            app_base_url: app_base_url.to_string(),
            frontend_base_url: frontend_base_url.to_string(),
            app_session_secret: None,
            github_client_id: None,
            github_client_secret: None,
            discord_client_id: None,
            discord_client_secret: None,
            admin_api_token: None,
            github_token: None,
            scheduler_enabled: false,
            recompute_interval_secs: 86_400,
            mcp_write_limit_per_hour: 60,
            mcp_log_usage_cooldown_secs: 900,
            mcp_negative_signal_window_hours: 24,
            active_signal_min_reputation: 0.45,
            active_signal_default_consensus: 2,
            active_signal_severe_consensus: 3,
            semantic_search_enabled: false,
        }
    }
}
