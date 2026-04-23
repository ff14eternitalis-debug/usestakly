use std::sync::Arc;

use chrono::{DateTime, Utc};
use http::request::Parts;
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
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::AppState,
    domain::reference::SearchFilter,
    mcp::auth::verify_bearer,
    services::{
        quality::scoring::load_v1,
        repos::{self as repos_service, RepoSearchFilters},
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

        let results = repos_service::search_github_repos(&self.state.db, &filters)
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
}

#[tool_handler(
    name = "usestakly-mcp",
    instructions = "UseStakly MCP — query a scored registry of public GitHub repos. \
                    Always call `search_github_repos` before generating code that pulls in \
                    a dependency, then `get_repo_quality_context` to confirm the pick. \
                    Include the returned provenance string when you write the code."
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
    let row: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id FROM external_artifacts
        WHERE source = 'github'
          AND github_owner = $1
          AND github_repo = $2
        LIMIT 1
        "#,
    )
    .bind(owner)
    .bind(name)
    .fetch_optional(db)
    .await
    .map_err(|e| ErrorData::internal_error(format!("db error: {e}"), None))?;
    Ok(row.map(|(id,)| id))
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
    StreamableHttpService::new(
        move || Ok(McpServer::new(state.clone())),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    )
}
