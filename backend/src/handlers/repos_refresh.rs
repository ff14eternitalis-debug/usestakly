use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use axum::{Json, extract::State, http::HeaderMap};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::quality_display::IngestionStatus,
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::recompute_external_artifact,
        repos::{
            RefreshLimitConfig, RefreshLimitsOutcome, STATUS_COMPLETED, STATUS_THROTTLED,
            check_refresh_limits, get_repo_profile, record_refresh_event,
        },
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
    headers: HeaderMap,
    axum::extract::Path(repo_id): axum::extract::Path<Uuid>,
) -> Result<Json<RefreshRepoResponse>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;

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

    if refresh_on_cooldown_memory(repo_id, state.config.repo_refresh_cooldown_secs) {
        return cached_refresh_response(&state, repo_id, false).await;
    }

    let limits = RefreshLimitConfig {
        user_per_hour: state.config.repo_refresh_user_limit_per_hour,
        repo_cooldown_secs: state.config.repo_refresh_cooldown_secs,
    };

    match check_refresh_limits(&state.db, user.id, repo_id, &limits).await? {
        RefreshLimitsOutcome::Allowed => {}
        RefreshLimitsOutcome::Throttled(reason) => {
            record_refresh_event(
                &state.db,
                user.id,
                repo_id,
                STATUS_THROTTLED,
                Some(reason.as_str()),
            )
            .await?;
            return cached_refresh_response(&state, repo_id, false).await;
        }
    }

    let client = build_client(token)?;
    ingest_repo(&client, &state.db, &state.config, &owner, &name).await?;
    recompute_external_artifact(&state.db, Some(&state.config), repo_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    record_refresh_event(&state.db, user.id, repo_id, STATUS_COMPLETED, None).await?;
    mark_refresh_memory(repo_id);

    cached_refresh_response(&state, repo_id, true).await
}

async fn cached_refresh_response(
    state: &AppState,
    repo_id: Uuid,
    refreshed: bool,
) -> Result<Json<RefreshRepoResponse>, ApiError> {
    let profile = get_repo_profile(&state.db, &state.config, repo_id).await?;
    Ok(Json(RefreshRepoResponse {
        refreshed,
        artifact_id: repo_id,
        structural_signals_at: profile.vitality_inputs.structural_signals_at,
        ingestion_status: profile.ingestion_status,
    }))
}

fn refresh_on_cooldown_memory(artifact_id: Uuid, cooldown_secs: u64) -> bool {
    let map = REFRESH_COOLDOWN.get_or_init(|| Mutex::new(HashMap::new()));
    let guard = map.lock().expect("refresh cooldown mutex");
    guard.get(&artifact_id).is_some_and(|last| {
        Utc::now().signed_duration_since(*last) < chrono::Duration::seconds(cooldown_secs as i64)
    })
}

fn mark_refresh_memory(artifact_id: Uuid) {
    let map = REFRESH_COOLDOWN.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = map.lock().expect("refresh cooldown mutex");
    guard.insert(artifact_id, Utc::now());
}
