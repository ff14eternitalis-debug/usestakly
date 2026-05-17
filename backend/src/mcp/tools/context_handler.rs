use http::request::Parts;
use rmcp::ErrorData;

use crate::{
    app::AppState,
    mcp::{
        auth::verify_bearer,
        tools::{RepoContextOutput, RepoContextParams, into_context_output, map_api_error},
    },
    services::{quality::load_v2, repos as repos_service},
};

use super::{map_anyhow, resolve_artifact_id};

pub async fn handle_get_repo_quality_context(
    state: &AppState,
    p: RepoContextParams,
    parts: Parts,
) -> Result<RepoContextOutput, ErrorData> {
    verify_bearer(&state.db, &parts).await?;

    let owner = p.owner.trim();
    let name = p.name.trim();
    if owner.is_empty() || name.is_empty() {
        return Err(ErrorData::invalid_params(
            "owner and name are required",
            None,
        ));
    }

    let artifact_id = resolve_artifact_id(&state.db, owner, name)
        .await?
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("repo not ingested: {owner}/{name}"), None)
        })?;

    let profile = repos_service::get_repo_profile(&state.db, &state.config, artifact_id)
        .await
        .map_err(map_api_error)?;

    let formula_version = load_v2().map_err(map_anyhow)?.meta.version;
    Ok(into_context_output(profile, formula_version))
}
