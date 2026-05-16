use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    domain::quality_display::IngestionStatus,
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::recompute_external_artifact,
        repos::get_repo_profile,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRepoResponse {
    pub refreshed: bool,
    pub artifact_id: Uuid,
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub ingestion_status: IngestionStatus,
}

static REFRESH_COOLDOWN: OnceLock<Mutex<HashMap<Uuid, DateTime<Utc>>>> = OnceLock::new();

pub async fn refresh_repo(
    State(state): State<AppState>,
    axum::extract::Path(repo_id): axum::extract::Path<Uuid>,
) -> Result<Json<RefreshRepoResponse>, ApiError> {
    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("GitHub refresh disabled (set GITHUB_TOKEN)"))?;

    let owner_name: Option<(String, String)> = sqlx::query_as(
        r#"
        SELECT github_owner, github_repo
        FROM external_artifacts
        WHERE id = $1 AND source = 'github'
        "#,
    )
    .bind(repo_id)
    .fetch_optional(&state.db)
    .await?;

    let (owner, name) = owner_name.ok_or_else(|| ApiError::not_found("Repo not found"))?;

    if refresh_on_cooldown(repo_id, state.config.repo_refresh_cooldown_secs) {
        let profile = get_repo_profile(&state.db, &state.config, repo_id).await?;
        return Ok(Json(RefreshRepoResponse {
            refreshed: false,
            artifact_id: repo_id,
            structural_signals_at: profile.vitality_inputs.structural_signals_at,
            ingestion_status: profile.ingestion_status,
        }));
    }

    let client = build_client(token)?;
    ingest_repo(&client, &state.db, &state.config, &owner, &name).await?;
    recompute_external_artifact(&state.db, Some(&state.config), repo_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    mark_refresh(repo_id);

    let profile = get_repo_profile(&state.db, &state.config, repo_id).await?;
    Ok(Json(RefreshRepoResponse {
        refreshed: true,
        artifact_id: repo_id,
        structural_signals_at: profile.vitality_inputs.structural_signals_at,
        ingestion_status: profile.ingestion_status,
    }))
}

fn refresh_on_cooldown(artifact_id: Uuid, cooldown_secs: u64) -> bool {
    let map = REFRESH_COOLDOWN.get_or_init(|| Mutex::new(HashMap::new()));
    let guard = map.lock().expect("refresh cooldown mutex");
    guard.get(&artifact_id).is_some_and(|last| {
        Utc::now().signed_duration_since(*last) < chrono::Duration::seconds(cooldown_secs as i64)
    })
}

fn mark_refresh(artifact_id: Uuid) {
    let map = REFRESH_COOLDOWN.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = map.lock().expect("refresh cooldown mutex");
    guard.insert(artifact_id, Utc::now());
}
