use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::{
        quality::CreateSignalRequest,
        reference::SearchFilter,
        repo::{RepoProfile, RepoSearchResult},
    },
    services::{
        ingestion::github::{build_client, ingest_repo, parse_github_repo_input},
        quality::{RecordSignalInput, recompute_all_scores_with_config, record_signal},
        reputation,
        repo_owners,
        repos::{RepoSearchFilters, find_github_artifact_id, get_repo_profile, get_repo_signals, search_github_repos},
        signal_events,
        signal_reviews,
    },
};

#[derive(Debug, Deserialize)]
pub struct RepoSearchQuery {
    pub q: Option<String>,
    #[serde(default)]
    pub filter: SearchFilter,
    pub language: Option<String>,
    pub license: Option<String>,
    pub stars_min: Option<i32>,
    #[serde(default)]
    pub include_archived: bool,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSearchResponse {
    pub filter: SearchFilter,
    pub items: Vec<RepoSearchResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRepoRequest {
    pub repo: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRepoResponse {
    pub artifact_id: Uuid,
    pub already_indexed: bool,
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
    pub default_branch: Option<String>,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub formula_version: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DisputeSignalRequest {
    #[validate(length(min = 10, max = 2000))]
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoViewerState {
    pub can_dispute_signals: bool,
    pub visible_signals: Vec<crate::domain::repo::RepoSignal>,
}

pub async fn search_repos(
    State(state): State<AppState>,
    Query(query): Query<RepoSearchQuery>,
) -> Result<Json<RepoSearchResponse>, ApiError> {
    let filters = RepoSearchFilters {
        query: normalize(query.q),
        filter: query.filter,
        language: normalize(query.language),
        license_spdx: normalize(query.license),
        stars_min: query.stars_min.filter(|v| *v >= 0),
        include_archived: query.include_archived,
        limit: query.limit,
    };
    let items = search_github_repos(&state.db, &filters).await?;
    Ok(Json(RepoSearchResponse {
        filter: filters.filter,
        items,
    }))
}

pub async fn get_repo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RepoProfile>, ApiError> {
    let profile = get_repo_profile(&state.db, id).await?;
    Ok(Json(profile))
}

pub async fn add_repo(
    State(state): State<AppState>,
    Json(payload): Json<AddRepoRequest>,
) -> Result<Json<AddRepoResponse>, ApiError> {
    let (owner, name) = parse_github_repo_input(&payload.repo)?;
    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("Repo ingestion disabled (set GITHUB_TOKEN)"))?;

    let already_indexed = find_github_artifact_id(&state.db, &owner, &name)
        .await?
        .is_some();
    let client = build_client(token)?;
    let (artifact_id, meta) = ingest_repo(&client, &state.db, &owner, &name).await?;
    let report = recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;

    Ok(Json(AddRepoResponse {
        artifact_id,
        already_indexed,
        owner: meta.owner.clone(),
        name: meta.name.clone(),
        full_name: format!("{}/{}", meta.owner, meta.name),
        html_url: meta.html_url,
        description: meta.description,
        language: meta.language,
        license_spdx: meta.license_spdx,
        topics: meta.topics,
        stars_count: meta.stars_count,
        forks_count: meta.forks_count,
        open_issues_count: meta.open_issues_count,
        subscribers_count: meta.subscribers_count,
        archived: meta.archived,
        default_branch: meta.default_branch,
        last_commit_at: meta.last_commit_at,
        formula_version: report.formula_version,
    }))
}

pub async fn create_repo_signal(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(repo_id): Path<Uuid>,
    Json(payload): Json<CreateSignalRequest>,
) -> Result<Json<crate::domain::quality::QualitySignalRecord>, ApiError> {
    if payload.signal.is_passive() {
        return Err(ApiError::bad_request(
            "Only active signals can be submitted on repos",
        ));
    }

    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let reputation = reputation::get_user_reputation(&state.db, user.id).await?;
    if !reputation.active_signal_eligible(state.config.active_signal_min_reputation) {
        return Err(ApiError::forbidden(format!(
            "Active signals require a reputation >= {:.2}, at least 5 passive signals, and a 7-day-old account",
            state.config.active_signal_min_reputation
        )));
    }

    let record = record_signal(
        &state.db,
        RecordSignalInput {
            artifact_kind: crate::domain::quality::ArtifactKind::External,
            snippet_id: None,
            external_artifact_id: Some(repo_id),
            signal: payload.signal,
            review_status: if matches!(payload.signal, crate::domain::quality::SignalKind::SecurityIssue) {
                "pending".to_string()
            } else {
                "accepted".to_string()
            },
            actor_user_id: Some(user.id),
            evidence_url: payload.evidence_url,
            evidence_description: payload.evidence_description,
            agent_context: payload.agent_context,
        },
    )
    .await?;
    signal_events::record_signal_event(
        &state.db,
        record.id,
        "submitted",
        Some(user.id),
        None,
    )
    .await?;
    recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;

    Ok(Json(record))
}

pub async fn get_repo_viewer_state(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(repo_id): Path<Uuid>,
) -> Result<Json<RepoViewerState>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let can_dispute =
        repo_owners::user_can_manage_repo_signal(&state.db, &state.config, user.id, repo_id)
            .await?;
    let signals = get_repo_signals(&state.db, repo_id).await?;

    Ok(Json(RepoViewerState {
        can_dispute_signals: can_dispute,
        visible_signals: if can_dispute {
            signals
        } else {
            signals
                .into_iter()
                .filter(|signal| signal.is_passive || signal.review_status == "accepted")
                .collect()
        },
    }))
}

pub async fn dispute_repo_signal(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path((repo_id, signal_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<DisputeSignalRequest>,
) -> Result<Json<crate::domain::quality::QualitySignalRecord>, ApiError> {
    payload.validate()?;
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;

    let can_manage =
        repo_owners::user_can_manage_repo_signal(&state.db, &state.config, user.id, repo_id)
            .await?;
    if !can_manage {
        return Err(ApiError::forbidden(
            "Only the GitHub repo owner can dispute active signals",
        ));
    }
    if !signal_reviews::signal_belongs_to_repo(&state.db, repo_id, signal_id).await? {
        return Err(ApiError::not_found("Signal not found for this repo"));
    }

    let record = signal_reviews::dispute_signal(&state.db, signal_id, user.id, payload.reason.trim())
        .await?;
    signal_events::record_signal_event(
        &state.db,
        record.id,
        "disputed",
        Some(user.id),
        Some(payload.reason.trim()),
    )
    .await?;
    recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;
    Ok(Json(record))
}

fn normalize(value: Option<String>) -> Option<String> {
    value
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
