use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    app::{AppState, error::ApiError},
    services::{
        ingestion::github::{build_client, ingest_repo},
        quality::{ScoringReport, recompute_all_scores_with_config},
        semantic_search,
        trust::{reputation, signal_events, signal_reviews},
    },
};

const ADMIN_TOKEN_HEADER: &str = "x-admin-token";

pub async fn recompute_scores(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ScoringReport>, ApiError> {
    require_admin_token(&state, &headers)?;
    let report = recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;
    Ok(Json(report))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSignalRequest {
    pub action: String,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct IngestGithubRequest {
    pub owner: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct IngestGithubResponse {
    pub id: Uuid,
    pub owner: String,
    pub name: String,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub default_branch: Option<String>,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub topics: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackfillEmbeddingsRequest {
    pub limit: Option<i64>,
    #[serde(default = "default_true")]
    pub only_missing: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackfillEmbeddingsResponse {
    pub updated: usize,
    pub limit: i64,
    pub only_missing: bool,
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PendingRepoSignalResponse {
    pub id: Uuid,
    pub repo_id: Uuid,
    pub owner: String,
    pub name: String,
    pub signal: String,
    pub review_status: String,
    pub actor_user_id: Option<Uuid>,
    pub disputed_by_user_id: Option<Uuid>,
    pub evidence_url: Option<String>,
    pub evidence_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reporter_score: Option<f64>,
    pub reporter_tier: Option<String>,
    pub reporter_usage_signal_count: Option<i64>,
    pub owner_dispute_score: Option<f64>,
    pub owner_dispute_tier: Option<String>,
    pub owner_dispute_usage_signal_count: Option<i64>,
    pub has_owner_dispute: bool,
    pub needs_strict_review: bool,
    pub suggested_action: String,
}

pub async fn ingest_github_repo(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<IngestGithubRequest>,
) -> Result<Json<IngestGithubResponse>, ApiError> {
    require_admin_token(&state, &headers)?;

    let owner = req.owner.trim();
    let name = req.name.trim();
    if owner.is_empty() || name.is_empty() {
        return Err(ApiError::bad_request("owner and name are required"));
    }
    if owner.contains('/') || name.contains('/') || owner.contains(' ') || name.contains(' ') {
        return Err(ApiError::bad_request(
            "owner and name must not contain '/' or whitespace",
        ));
    }

    let token = state
        .config
        .github_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("GitHub ingestion disabled (set GITHUB_TOKEN)"))?;

    let client = build_client(token)?;
    let (id, meta) = ingest_repo(&client, &state.db, &state.config, owner, name).await?;

    Ok(Json(IngestGithubResponse {
        id,
        owner: meta.owner,
        name: meta.name,
        stars_count: meta.stars_count,
        forks_count: meta.forks_count,
        open_issues_count: meta.open_issues_count,
        subscribers_count: meta.subscribers_count,
        archived: meta.archived,
        language: meta.language,
        license_spdx: meta.license_spdx,
        default_branch: meta.default_branch,
        last_commit_at: meta.last_commit_at,
        topics: meta.topics,
    }))
}

pub async fn backfill_repo_embeddings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<BackfillEmbeddingsRequest>,
) -> Result<Json<BackfillEmbeddingsResponse>, ApiError> {
    require_admin_token(&state, &headers)?;

    let limit = req.limit.unwrap_or(100).clamp(1, 500);
    let updated = semantic_search::backfill_repo_embeddings(
        &state.db,
        &state.config,
        limit,
        req.only_missing,
    )
    .await?;

    Ok(Json(BackfillEmbeddingsResponse {
        updated,
        limit,
        only_missing: req.only_missing,
    }))
}

pub async fn review_repo_signal(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(signal_id): Path<Uuid>,
    Json(req): Json<ReviewSignalRequest>,
) -> Result<Json<crate::domain::quality::QualitySignalRecord>, ApiError> {
    require_admin_token(&state, &headers)?;
    let status = match req.action.trim().to_ascii_lowercase().as_str() {
        "approve" | "accepted" => "accepted",
        "reject" | "rejected" => "rejected",
        "pending" => "pending",
        other => {
            return Err(ApiError::bad_request(format!(
                "invalid action '{other}' (expected approve, reject, or pending)"
            )));
        }
    };

    let record = signal_reviews::review_signal(
        &state.db,
        signal_id,
        status,
        None,
        req.note.as_deref().map(str::trim).filter(|s| !s.is_empty()),
    )
    .await?;
    signal_events::record_signal_event(
        &state.db,
        record.id,
        match status {
            "accepted" => "review_accepted",
            "rejected" => "review_rejected",
            _ => "review_pending",
        },
        None,
        req.note.as_deref().map(str::trim).filter(|s| !s.is_empty()),
    )
    .await?;
    let _ = recompute_all_scores_with_config(&state.db, Some(&state.config)).await?;
    Ok(Json(record))
}

pub async fn list_pending_repo_signals(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<PendingRepoSignalResponse>>, ApiError> {
    require_admin_token(&state, &headers)?;

    let mut rows: Vec<PendingRepoSignalResponse> = sqlx::query_as(
        r#"
        SELECT
          qs.id AS id,
          e.id AS repo_id,
          COALESCE(e.github_owner, '') AS owner,
          COALESCE(e.github_repo, '') AS name,
          qs.signal::text AS signal,
          qs.review_status AS review_status,
          qs.actor_user_id AS actor_user_id,
          qs.disputed_by_user_id AS disputed_by_user_id,
          qs.evidence_url AS evidence_url,
          qs.evidence_description AS evidence_description,
          qs.created_at AS created_at,
          NULL::float8 AS reporter_score,
          NULL::text AS reporter_tier,
          NULL::bigint AS reporter_usage_signal_count,
          NULL::float8 AS owner_dispute_score,
          NULL::text AS owner_dispute_tier,
          NULL::bigint AS owner_dispute_usage_signal_count,
          FALSE AS has_owner_dispute,
          FALSE AS needs_strict_review,
          ''::text AS suggested_action
        FROM quality_signals qs
        JOIN external_artifacts e ON e.id = qs.external_artifact_id
        WHERE qs.is_passive = FALSE
          AND (
            qs.review_status IN ('pending', 'disputed')
            OR qs.disputed_at IS NOT NULL
          )
        ORDER BY qs.created_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    let reputations = reputation::list_user_reputations(&state.db).await?;
    for row in &mut rows {
        if let Some(actor_user_id) = row.actor_user_id {
            if let Some(rep) = reputations.get(&actor_user_id) {
                row.reporter_score = Some(rep.score);
                row.reporter_tier = Some(rep.tier.as_str().to_string());
                row.reporter_usage_signal_count = Some(rep.usage_signal_count());
                row.needs_strict_review = rep.requires_strict_active_review()
                    || matches!(row.signal.as_str(), "security_issue");
                row.suggested_action = if rep.requires_strict_active_review() {
                    "needs_manual_review".to_string()
                } else if matches!(row.signal.as_str(), "security_issue") {
                    "review_with_evidence".to_string()
                } else {
                    "standard_review".to_string()
                };
            } else {
                row.needs_strict_review = true;
                row.suggested_action = "needs_manual_review".to_string();
            }
        } else {
            row.needs_strict_review = true;
            row.suggested_action = "needs_manual_review".to_string();
        }

        if let Some(disputed_by_user_id) = row.disputed_by_user_id {
            row.has_owner_dispute = true;
            if let Some(rep) = reputations.get(&disputed_by_user_id) {
                row.owner_dispute_score = Some(rep.score);
                row.owner_dispute_tier = Some(rep.tier.as_str().to_string());
                row.owner_dispute_usage_signal_count = Some(rep.usage_signal_count());
                row.suggested_action = if rep.review_weight() >= 0.8 {
                    "review_owner_dispute_priority".to_string()
                } else if rep.requires_strict_active_review() {
                    "review_owner_dispute_low_trust".to_string()
                } else {
                    "review_owner_dispute_standard".to_string()
                };
            } else {
                row.suggested_action = "review_owner_dispute_standard".to_string();
            }
        }
    }

    Ok(Json(rows))
}

fn require_admin_token(state: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    let expected = state
        .config
        .admin_api_token
        .as_deref()
        .ok_or_else(|| ApiError::forbidden("Admin API not enabled (set ADMIN_API_TOKEN)"))?;
    let provided = headers
        .get(ADMIN_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiError::forbidden("Missing admin token"))?;
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return Err(ApiError::forbidden("Invalid admin token"));
    }
    Ok(())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn default_true() -> bool {
    true
}
