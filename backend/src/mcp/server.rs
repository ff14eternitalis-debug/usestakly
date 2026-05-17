use std::sync::Arc;

use http::request::Parts;
use rmcp::{
    ErrorData, ServerHandler,
    handler::server::{
        tool::Extension,
        wrapper::{Json, Parameters},
    },
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, session::local::LocalSessionManager,
        tower::StreamableHttpService,
    },
};
use serde_json::json;

use crate::{
    app::AppState,
    domain::quality::ArtifactKind,
    mcp::auth::{verify_agent, verify_bearer},
    services::{
        quality::{RecordSignalInput, load_v2, recompute_all_scores_with_config, record_signal},
        recommendations::recommend_for_use_case,
        repos::{self as repos_service, RepoSearchFilters, RepoSort},
        trust::agent_token_events,
        use_case_watches, watchlist,
    },
};

pub use crate::mcp::tools::{
    LogUsageOutput, LogUsageParams, Provenance, RadarBrief, RecommendReposOutput,
    RecommendReposParams, RecommendationFallback, RecommendationSections, RepoCandidate,
    RepoContextOutput, RepoContextParams, RepoRecommendation, SearchReposOutput, SearchReposParams,
    SignalSummary, VitalityInputsOutput, WatchRepoOutput, WatchRepoParams, WatchUseCaseMatchOutput,
    WatchUseCaseOutput, WatchUseCaseParams,
};

use crate::mcp::tools::{
    build_recommendations_from_use_case, build_use_case_service_query, ensure_github_artifact,
    into_context_output, into_repo_candidate, into_watch_use_case_output, map_anyhow,
    map_api_error, mcp_allowed_hosts, normalize_topics, parse_filter, parse_passive_outcome,
    parse_risk_tolerance, recommendation_sections, repo_matches_topics, resolve_artifact_id,
};

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

        let results =
            repos_service::search_github_repos(&self.state.db, &self.state.config, &filters)
                .await
                .map_err(map_api_error)?;

        let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
        let scored_at = results
            .iter()
            .filter_map(|r| r.quality.as_ref().map(|q| q.computed_at))
            .max();

        let candidates: Vec<RepoCandidate> = results.into_iter().map(into_repo_candidate).collect();

        Ok(Json(SearchReposOutput {
            provenance: Provenance {
                source: "usestakly://registry/github".to_string(),
                formula_version,
                scored_at,
            },
            filter_used: filter.as_str().to_string(),
            sort_used: sort.as_str().to_string(),
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

        let ecosystem = p.ecosystem.as_deref().or(p.language.as_deref());
        let normalized_topics = normalize_topics(&p.must_have_topics);
        let risk_tolerance = parse_risk_tolerance(p.risk_tolerance.as_deref());
        let service_query = build_use_case_service_query(query, ecosystem, &normalized_topics);
        let mut report = recommend_for_use_case(
            &self.state.db,
            &self.state.config,
            &service_query,
            risk_tolerance.as_str(),
            (p.limit.unwrap_or(5).clamp(1, 10) * 4).clamp(10, 40),
        )
        .await
        .map_err(map_api_error)?;
        if !normalized_topics.is_empty() {
            report.recommendations.retain(|recommendation| {
                repo_matches_topics(&recommendation.repo, &normalized_topics)
            });
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

        Ok(Json(RecommendReposOutput {
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

        let profile =
            repos_service::get_repo_profile(&self.state.db, &self.state.config, artifact_id)
                .await
                .map_err(map_api_error)?;

        let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
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

        let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
        let profile =
            repos_service::get_repo_profile(&self.state.db, &self.state.config, artifact_id)
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

        let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
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

    #[tool(
        name = "watch_use_case",
        description = "Create a UseStakly watch for a natural-language dependency need, \
                       such as `testing tools for TypeScript` or `emerging auth libraries`. \
                       Use this when the user wants ongoing radar monitoring for a need, \
                       not just one repository."
    )]
    async fn watch_use_case(
        &self,
        Parameters(p): Parameters<WatchUseCaseParams>,
        Extension(parts): Extension<Parts>,
    ) -> Result<Json<WatchUseCaseOutput>, ErrorData> {
        let agent = verify_agent(&self.state.db, &parts).await?;
        let need = p.need.trim();
        if need.is_empty() {
            return Err(ErrorData::invalid_params("need is required", None));
        }
        let risk_tolerance = parse_risk_tolerance(p.risk_tolerance.as_deref());

        agent_token_events::enforce_write_quota(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            agent_token_events::REJECTION_TOOL_WATCH_USE_CASE,
            "use-case",
            need,
            self.state.config.mcp_write_limit_per_hour,
        )
        .await
        .map_err(map_api_error)?;

        let watch = use_case_watches::create_watch(
            &self.state.db,
            &self.state.config,
            agent.user_id,
            need,
            p.label,
            risk_tolerance.as_str(),
        )
        .await
        .map_err(map_api_error)?;

        agent_token_events::record_watch_use_case(
            &self.state.db,
            agent.token_id,
            agent.user_id,
            &watch.label,
            &watch.query_text,
        )
        .await
        .map_err(map_api_error)?;

        let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
        Ok(Json(into_watch_use_case_output(
            watch,
            formula_version,
            None,
        )))
    }
}

#[tool_handler(
    name = "usestakly-mcp",
    instructions = "UseStakly MCP — query a scored registry of public GitHub repos. \
                    Always call `search_github_repos` before generating code that pulls in \
                    a dependency, then `get_repo_quality_context` to confirm the pick. \
                    After trying a repo, call `log_usage`. Use `watch_repo` for a dependency \
                    and `watch_use_case` for an ongoing need/radar watch. Write calls are rate-limited per token and duplicate \
                    `log_usage` events are intentionally throttled. Include the returned provenance \
                    string when you write the code."
)]
impl ServerHandler for McpServer {}

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

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    use crate::config::AppConfig;
    use crate::domain::{
        quality::SignalKind,
        reference::{QualityContext, SearchFilter},
        repo::{
            RepoCategory, RepoProfile, RepoRadarSnapshot, RepoSearchResult, RepoSignal,
            VitalityInputs,
        },
    };
    use crate::mcp::tools::{RadarBrief, RepoRecommendation};
    use crate::mcp::tools::{
        RiskTolerance, build_recommendation_fallback, build_recommendation_query,
        build_recommendations, build_recommendations_from_use_case, into_context_output,
        into_watch_use_case_output, mcp_allowed_hosts, normalize_topics, parse_filter,
        parse_passive_outcome, recommendation_sections, repo_matches_intent,
    };
    use crate::services::recommendations::parse_intent;

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
        let config = test_config("https://mcp.usestakly.com", "https://www.usestakly.com");

        let hosts = mcp_allowed_hosts(&config);

        assert!(hosts.contains(&"localhost".to_string()));
        assert!(hosts.contains(&"127.0.0.1".to_string()));
        assert!(hosts.contains(&"mcp.usestakly.com".to_string()));
        assert!(hosts.contains(&"www.usestakly.com".to_string()));
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
                    formula_version: "v2.0".to_string(),
                    freshness: Some(0.91),
                    adoption: Some(0.98),
                    reliability: Some(0.84),
                    abandonment: Some(0.08),
                    vitality: Some(0.95),
                    overall: Some(0.9),
                    resolve_count: 12,
                    build_success_count: 8,
                    build_failure_count: 1,
                    regret_count: 0,
                    flags: vec!["deprecated".to_string()],
                    computed_at,
                }),
                categories: Vec::new(),
                radar: None,
                recommendation_explanation: None,
            },
            subscribers_count: 6_400,
            default_branch: Some("main".to_string()),
            priors_fetched_at: Some(computed_at),
            vitality_inputs: VitalityInputs {
                structural_signals_at: Some(computed_at),
                distinct_contributors_90d: Some(120),
                commits_30d: Some(450),
                has_ci: Some(true),
                releases_count: Some(40),
                last_release_at: Some(computed_at),
                owner_last_activity_at: Some(computed_at),
                owner_inactive_days: Some(0),
            },
            dimension_states: Vec::new(),
            proof_tier: "community_backed".to_string(),
            ingestion_status: crate::domain::quality_display::IngestionStatus {
                priors_fetched_at: Some(computed_at),
                structural_signals_at: Some(computed_at),
                structural_stale: false,
                structural_complete: true,
                partial_fields: Vec::new(),
            },
            score_snapshot: None,
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

        let output = into_context_output(profile, "v2.0".to_string());

        assert_eq!(
            output.provenance.source,
            "usestakly://registry/github/facebook/react"
        );
        assert_eq!(output.provenance.formula_version, "v2.0");
        assert_eq!(output.quality_vitality, Some(0.95));
        assert_eq!(output.vitality_inputs.has_ci, Some(true));
        assert_eq!(output.vitality_inputs.distinct_contributors_90d, Some(120));
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
                formula_version: "v2.0".to_string(),
                freshness: Some(0.92),
                adoption: Some(0.0),
                reliability: Some(0.5),
                abandonment: Some(0.04),
                vitality: Some(0.78),
                overall: Some(0.71),
                resolve_count: 0,
                build_success_count: 1,
                build_failure_count: 0,
                regret_count: 0,
                flags: Vec::new(),
                computed_at,
            }),
            categories: Vec::new(),
            radar: None,
            recommendation_explanation: None,
        }];

        let recommendations = build_recommendations(results, RiskTolerance::Medium);

        assert_eq!(recommendations.len(), 1);
        assert_eq!(recommendations[0].rank, 1);
        assert_eq!(recommendations[0].full_name, "example/typed-orm");
        assert_eq!(recommendations[0].topics, vec!["orm", "typescript"]);
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

    #[test]
    fn recommendations_include_radar_maturity_and_caveats() {
        let computed_at = Utc.with_ymd_and_hms(2026, 4, 25, 10, 0, 0).unwrap();
        let artifact_id = Uuid::parse_str("66666666-6666-4666-8666-666666666666").unwrap();
        let results = vec![RepoSearchResult {
            artifact_id,
            owner: "example".to_string(),
            name: "new-test-runner".to_string(),
            full_name: "example/new-test-runner".to_string(),
            html_url: "https://github.com/example/new-test-runner".to_string(),
            description: Some("A fast testing tool.".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: Some("MIT".to_string()),
            topics: vec!["testing".to_string()],
            stars_count: 420,
            forks_count: 22,
            open_issues_count: 12,
            archived: false,
            last_commit_at: Some(computed_at),
            quality: Some(QualityContext {
                formula_version: "v2.0".to_string(),
                freshness: Some(0.9),
                adoption: Some(0.0),
                reliability: Some(0.5),
                abandonment: Some(0.08),
                vitality: Some(0.7),
                overall: Some(0.62),
                resolve_count: 0,
                build_success_count: 0,
                build_failure_count: 0,
                regret_count: 0,
                flags: Vec::new(),
                computed_at,
            }),
            categories: Vec::new(),
            radar: Some(RepoRadarSnapshot {
                maturity_band: "emerging".to_string(),
                radar_relevance: 0.72,
                trend_signal: 0.88,
                explanation: serde_json::json!({ "reasons": ["clear_category", "recent_activity"] }),
            }),
            recommendation_explanation: None,
        }];

        let recommendations = build_recommendations(results, RiskTolerance::Medium);

        assert_eq!(
            recommendations[0]
                .radar
                .as_ref()
                .map(|radar| radar.maturity_band.as_str()),
            Some("emerging")
        );
        assert!(
            recommendations[0]
                .caveats
                .iter()
                .any(|caveat| caveat.contains("Radar marks this repo as emerging"))
        );
        assert!(
            recommendations[0]
                .next_actions
                .iter()
                .any(|action| action.contains("watch_repo"))
        );
    }

    #[test]
    fn recommendation_sections_split_stable_and_emerging_picks() {
        let stable = RepoRecommendation {
            rank: 1,
            owner: "established".to_string(),
            name: "orm".to_string(),
            full_name: "established/orm".to_string(),
            html_url: "https://github.com/established/orm".to_string(),
            description: None,
            language: Some("TypeScript".to_string()),
            topics: vec!["orm".to_string()],
            stars_count: 20_000,
            quality_overall: Some(0.82),
            quality_freshness: Some(0.9),
            quality_adoption: Some(0.2),
            quality_reliability: Some(0.75),
            quality_abandonment: Some(0.05),
            quality_vitality: Some(0.9),
            flags: Vec::new(),
            radar: Some(RadarBrief {
                maturity_band: "established".to_string(),
                radar_relevance: 0.9,
                trend_signal: 0.5,
                summary: "Radar: established baseline.".to_string(),
            }),
            reasons: Vec::new(),
            caveats: Vec::new(),
            next_actions: Vec::new(),
        };
        let emerging = RepoRecommendation {
            rank: 2,
            owner: "new".to_string(),
            name: "orm".to_string(),
            full_name: "new/orm".to_string(),
            html_url: "https://github.com/new/orm".to_string(),
            description: None,
            language: Some("TypeScript".to_string()),
            topics: vec!["orm".to_string()],
            stars_count: 500,
            quality_overall: Some(0.63),
            quality_freshness: Some(0.95),
            quality_adoption: Some(0.0),
            quality_reliability: Some(0.5),
            quality_abandonment: Some(0.08),
            quality_vitality: Some(0.82),
            flags: Vec::new(),
            radar: Some(RadarBrief {
                maturity_band: "emerging".to_string(),
                radar_relevance: 0.8,
                trend_signal: 0.88,
                summary: "Radar: promising emerging repo.".to_string(),
            }),
            reasons: Vec::new(),
            caveats: Vec::new(),
            next_actions: Vec::new(),
        };

        let sections = recommendation_sections(vec![stable, emerging], Vec::new());

        assert_eq!(sections.stable_picks.len(), 1);
        assert_eq!(sections.stable_picks[0].full_name, "established/orm");
        assert_eq!(sections.emerging_picks.len(), 1);
        assert_eq!(sections.emerging_picks[0].full_name, "new/orm");
        assert_eq!(sections.fallback_candidates, Vec::<String>::new());
    }

    #[test]
    fn watch_use_case_output_exposes_watch_summary_and_matches() {
        let created_at = Utc.with_ymd_and_hms(2026, 5, 3, 9, 30, 0).unwrap();
        let watch = crate::services::use_case_watches::UseCaseWatch {
            id: Uuid::parse_str("99999999-9999-4999-8999-999999999999").unwrap(),
            query_text: "testing tools for TypeScript".to_string(),
            label: "Veille Testing".to_string(),
            normalized_intent: "Testing".to_string(),
            categories: vec!["testing".to_string()],
            topics: vec!["test".to_string(), "testing-tools".to_string()],
            languages: vec!["TypeScript".to_string()],
            risk_tolerance: "medium".to_string(),
            enabled: true,
            match_count: 1,
            top_matches: vec![crate::services::use_case_watches::UseCaseWatchMatch {
                artifact_id: Uuid::parse_str("88888888-8888-4888-8888-888888888888").unwrap(),
                full_name: "vitest-dev/vitest".to_string(),
                language: Some("TypeScript".to_string()),
                match_score: 0.82,
                quality_score: Some(0.74),
            }],
            created_at,
        };

        let output = into_watch_use_case_output(watch, "v2.0".to_string(), Some(created_at));

        assert_eq!(output.provenance.source, "usestakly://watch/use-case");
        assert_eq!(output.provenance.formula_version, "v2.0");
        assert_eq!(
            output.watch_id.to_string(),
            "99999999-9999-4999-8999-999999999999"
        );
        assert_eq!(output.label, "Veille Testing");
        assert_eq!(output.initial_matches, 1);
        assert_eq!(output.top_matches[0].full_name, "vitest-dev/vitest");
    }

    #[test]
    fn mcp_recommendations_preserve_use_case_service_reason() {
        let computed_at = Utc.with_ymd_and_hms(2026, 5, 3, 11, 0, 0).unwrap();
        let recommendation = crate::services::recommendations::UseCaseRecommendation {
            repo: RepoSearchResult {
                artifact_id: Uuid::parse_str("77777777-7777-4777-8777-777777777777").unwrap(),
                owner: "vitest-dev".to_string(),
                name: "vitest".to_string(),
                full_name: "vitest-dev/vitest".to_string(),
                html_url: "https://github.com/vitest-dev/vitest".to_string(),
                description: Some("Testing framework powered by Vite.".to_string()),
                language: Some("TypeScript".to_string()),
                license_spdx: Some("MIT".to_string()),
                topics: vec!["testing".to_string()],
                stars_count: 16_000,
                forks_count: 800,
                open_issues_count: 120,
                archived: false,
                last_commit_at: Some(computed_at),
                quality: Some(QualityContext {
                    formula_version: "v2.0".to_string(),
                    freshness: Some(0.95),
                    adoption: Some(0.1),
                    reliability: Some(0.55),
                    abandonment: Some(0.05),
                    vitality: Some(0.83),
                    overall: Some(0.74),
                    resolve_count: 2,
                    build_success_count: 4,
                    build_failure_count: 0,
                    regret_count: 0,
                    flags: Vec::new(),
                    computed_at,
                }),
                categories: vec![RepoCategory {
                    category: "testing".to_string(),
                    confidence: 0.95,
                    source: "github_metadata+readme".to_string(),
                    evidence: serde_json::json!({}),
                }],
                radar: None,
                recommendation_explanation: None,
            },
            match_score: 0.9,
            recommendation_score: 0.81,
            risk: "medium".to_string(),
            reason: "Service says this matches the testing intent.".to_string(),
            matched_topics: vec!["testing".to_string()],
        };

        let recommendations =
            build_recommendations_from_use_case(vec![recommendation], RiskTolerance::Medium);

        assert_eq!(recommendations.len(), 1);
        assert_eq!(recommendations[0].full_name, "vitest-dev/vitest");
        assert!(
            recommendations[0]
                .reasons
                .iter()
                .any(|reason| reason == "Service says this matches the testing intent.")
        );
    }

    #[test]
    fn recommendation_query_infers_testing_terms_from_natural_need() {
        let intent = parse_intent("outil de test JavaScript");
        let query =
            build_recommendation_query("outil de test JavaScript", None, &Vec::new(), &intent);

        assert!(query.contains("test"));
        assert!(query.contains("testing"));
    }

    #[test]
    fn recommendation_intent_filter_keeps_testing_and_rejects_unrelated_javascript() {
        let intent = parse_intent("outil de test JavaScript");
        let computed_at = Utc.with_ymd_and_hms(2026, 4, 24, 8, 0, 0).unwrap();
        let testing_repo = RepoSearchResult {
            artifact_id: Uuid::parse_str("44444444-4444-4444-8444-444444444444").unwrap(),
            owner: "vitest-dev".to_string(),
            name: "vitest".to_string(),
            full_name: "vitest-dev/vitest".to_string(),
            html_url: "https://github.com/vitest-dev/vitest".to_string(),
            description: Some("Next generation testing framework powered by Vite.".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: Some("MIT".to_string()),
            topics: vec!["test".to_string(), "testing-tools".to_string()],
            stars_count: 16_000,
            forks_count: 900,
            open_issues_count: 100,
            archived: false,
            last_commit_at: Some(computed_at),
            quality: None,
            categories: vec![RepoCategory {
                category: "testing".to_string(),
                confidence: 0.98,
                source: "github_metadata+readme".to_string(),
                evidence: serde_json::json!({}),
            }],
            radar: None,
            recommendation_explanation: None,
        };
        let unrelated_repo = RepoSearchResult {
            artifact_id: Uuid::parse_str("55555555-5555-4555-8555-555555555555").unwrap(),
            owner: "remotion-dev".to_string(),
            name: "remotion".to_string(),
            full_name: "remotion-dev/remotion".to_string(),
            html_url: "https://github.com/remotion-dev/remotion".to_string(),
            description: Some("Make videos programmatically with React.".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: Some("MIT".to_string()),
            topics: vec![
                "javascript".to_string(),
                "typescript".to_string(),
                "react".to_string(),
                "video".to_string(),
            ],
            stars_count: 45_000,
            forks_count: 1_500,
            open_issues_count: 80,
            archived: false,
            last_commit_at: Some(computed_at),
            quality: None,
            categories: vec![RepoCategory {
                category: "video-tool".to_string(),
                confidence: 0.98,
                source: "github_metadata+readme".to_string(),
                evidence: serde_json::json!({}),
            }],
            radar: None,
            recommendation_explanation: None,
        };

        assert!(repo_matches_intent(&testing_repo, &intent));
        assert!(!repo_matches_intent(&unrelated_repo, &intent));
    }

    #[test]
    fn recommendation_filters_topics_and_builds_fallback() {
        let topics = normalize_topics(&[
            "React".to_string(),
            "#Data-Grid".to_string(),
            "react".to_string(),
        ]);
        assert_eq!(topics, vec!["react", "data-grid"]);

        let fallback =
            build_recommendation_fallback("table grid", Some("React"), &topics, RiskTolerance::Low);

        assert!(fallback.message.contains("No indexed repo matched"));
        assert!(
            fallback
                .add_repo_candidates
                .iter()
                .any(|item| item.contains("table grid React react data-grid"))
        );
        assert!(
            fallback
                .next_actions
                .iter()
                .any(|item| item.contains("low"))
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
            app_notification_secret: None,
            github_client_id: None,
            github_client_secret: None,
            discord_client_id: None,
            discord_client_secret: None,
            admin_api_token: None,
            github_token: None,
            email_smtp_host: "smtp-relay.brevo.com".to_string(),
            email_smtp_port: 587,
            email_smtp_username: None,
            email_smtp_password: None,
            email_from_address: "noreply@usestakly.com".to_string(),
            email_from_name: "UseStakly".to_string(),
            scheduler_enabled: false,
            recompute_interval_secs: 3_600,
            digest_interval_secs: 1_800,
            corpus_refresh_stale_secs: 3_600,
            ingest_max_repos_per_cycle: 40,
            scheduler_run_on_startup: false,
            mcp_auth_failure_limit_per_minute: 30,
            mcp_read_limit_per_minute: 120,
            mcp_write_limit_per_hour: 60,
            mcp_log_usage_cooldown_secs: 900,
            mcp_negative_signal_window_hours: 24,
            active_signal_min_reputation: 0.45,
            active_signal_default_consensus: 2,
            active_signal_severe_consensus: 3,
            semantic_search_enabled: false,
            structural_stale_secs: 172_800,
            repo_refresh_cooldown_secs: 900,
            repo_refresh_user_limit_per_hour: 10,
        }
    }
}
