use http::request::Parts;
use rmcp::ErrorData;
use serde_json::json;

use crate::{
    app::AppState,
    domain::quality::ArtifactKind,
    mcp::{
        auth::verify_agent,
        tools::{LogUsageOutput, LogUsageParams, Provenance, map_api_error},
    },
    services::{
        quality::{RecordSignalInput, load_v2, recompute_all_scores_with_config, record_signal},
        repos as repos_service,
        trust::agent_token_events,
    },
};

use super::{ensure_github_artifact, map_anyhow, parse_passive_outcome};

pub async fn handle_log_usage(
    state: &AppState,
    p: LogUsageParams,
    parts: Parts,
) -> Result<LogUsageOutput, ErrorData> {
    let agent = verify_agent(&state.db, &parts).await?;
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
        &state.db,
        agent.token_id,
        agent.user_id,
        agent_token_events::REJECTION_TOOL_LOG_USAGE,
        owner,
        name,
        state.config.mcp_write_limit_per_hour,
    )
    .await
    .map_err(map_api_error)?;
    agent_token_events::enforce_log_usage_guards(
        &state.db,
        agent.token_id,
        agent.user_id,
        owner,
        name,
        signal,
        notes,
        state.config.mcp_log_usage_cooldown_secs,
        state.config.mcp_negative_signal_window_hours,
    )
    .await
    .map_err(map_api_error)?;
    let artifact_id = ensure_github_artifact(state, owner, name).await?;

    let record = record_signal(
        &state.db,
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
        &state.db,
        agent.token_id,
        agent.user_id,
        owner,
        name,
        signal,
        notes,
    )
    .await
    .map_err(map_api_error)?;
    let report = recompute_all_scores_with_config(&state.db, Some(&state.config))
        .await
        .map_err(map_anyhow)?;

    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    let profile = repos_service::get_repo_profile(&state.db, &state.config, artifact_id)
        .await
        .map_err(map_api_error)?;
    let q = profile.repo.quality.as_ref();
    Ok(LogUsageOutput {
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
    })
}
