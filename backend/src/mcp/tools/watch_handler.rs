use http::request::Parts;
use rmcp::ErrorData;

use crate::{
    app::AppState,
    mcp::{
        auth::verify_agent,
        tools::{
            Provenance, WatchRepoOutput, WatchRepoParams, WatchUseCaseOutput, WatchUseCaseParams,
            into_watch_use_case_output, map_api_error,
        },
    },
    services::{quality::load_v2, trust::agent_token_events, use_case_watches, watchlist},
};

use super::{ensure_github_artifact, map_anyhow, parse_risk_tolerance};

pub async fn handle_watch_repo(
    state: &AppState,
    p: WatchRepoParams,
    parts: Parts,
) -> Result<WatchRepoOutput, ErrorData> {
    let agent = verify_agent(&state.db, &parts).await?;
    let owner = p.owner.trim();
    let name = p.name.trim();
    if owner.is_empty() || name.is_empty() {
        return Err(ErrorData::invalid_params(
            "owner and name are required",
            None,
        ));
    }

    agent_token_events::enforce_write_quota(
        &state.db,
        agent.token_id,
        agent.user_id,
        agent_token_events::REJECTION_TOOL_WATCH_REPO,
        owner,
        name,
        state.config.mcp_write_limit_per_hour,
    )
    .await
    .map_err(map_api_error)?;
    let artifact_id = ensure_github_artifact(state, owner, name).await?;
    watchlist::add_watch(&state.db, agent.user_id, artifact_id)
        .await
        .map_err(map_api_error)?;
    agent_token_events::record_watch_repo(&state.db, agent.token_id, agent.user_id, owner, name)
        .await
        .map_err(map_api_error)?;

    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    Ok(WatchRepoOutput {
        provenance: Provenance {
            source: format!("usestakly://registry/github/{owner}/{name}"),
            formula_version,
            scored_at: None,
        },
        owner: owner.to_string(),
        name: name.to_string(),
        artifact_id: artifact_id.to_string(),
        watching: true,
    })
}

pub async fn handle_watch_use_case(
    state: &AppState,
    p: WatchUseCaseParams,
    parts: Parts,
) -> Result<WatchUseCaseOutput, ErrorData> {
    let agent = verify_agent(&state.db, &parts).await?;
    let need = p.need.trim();
    if need.is_empty() {
        return Err(ErrorData::invalid_params("need is required", None));
    }
    let risk_tolerance = parse_risk_tolerance(p.risk_tolerance.as_deref());

    agent_token_events::enforce_write_quota(
        &state.db,
        agent.token_id,
        agent.user_id,
        agent_token_events::REJECTION_TOOL_WATCH_USE_CASE,
        "use-case",
        need,
        state.config.mcp_write_limit_per_hour,
    )
    .await
    .map_err(map_api_error)?;

    let watch = use_case_watches::create_watch(
        &state.db,
        &state.config,
        agent.user_id,
        need,
        p.label,
        risk_tolerance.as_str(),
    )
    .await
    .map_err(map_api_error)?;

    agent_token_events::record_watch_use_case(
        &state.db,
        agent.token_id,
        agent.user_id,
        &watch.label,
        &watch.query_text,
    )
    .await
    .map_err(map_api_error)?;

    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    Ok(into_watch_use_case_output(watch, formula_version, None))
}
