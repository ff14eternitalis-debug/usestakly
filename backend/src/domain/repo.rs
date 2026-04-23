use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::reference::QualityContext;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSearchResult {
    pub artifact_id: Uuid,
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
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub quality: Option<QualityContext>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSignalEvent {
    pub event_kind: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSignal {
    pub id: Uuid,
    pub signal: String,
    pub is_passive: bool,
    pub evidence_url: Option<String>,
    pub evidence_description: Option<String>,
    pub review_status: String,
    pub review_note: Option<String>,
    pub disputed_at: Option<DateTime<Utc>>,
    pub dispute_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub events: Vec<RepoSignalEvent>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoProfile {
    #[serde(flatten)]
    pub repo: RepoSearchResult,
    pub subscribers_count: i32,
    pub default_branch: Option<String>,
    pub priors_fetched_at: Option<DateTime<Utc>>,
    pub recent_signals: Vec<RepoSignal>,
}
