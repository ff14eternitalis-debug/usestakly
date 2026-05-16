use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::reference::QualityContext;

/// Why this repo appears in the current search/profile context (not radar JSON).
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationExplanation {
    pub included_because: Vec<String>,
    pub caveats: Vec<String>,
}

/// Current score row for profile (no time series without a dedicated snapshots table).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreSnapshot {
    pub formula_version: String,
    pub overall: Option<f64>,
    pub freshness: Option<f64>,
    pub adoption: Option<f64>,
    pub reliability: Option<f64>,
    pub abandonment: Option<f64>,
    pub vitality: Option<f64>,
    pub computed_at: DateTime<Utc>,
    pub previous_formula_version: Option<String>,
    pub previous_overall: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilterSummary {
    pub message_code: Option<String>,
}

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
    pub categories: Vec<RepoCategory>,
    pub radar: Option<RepoRadarSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_explanation: Option<RecommendationExplanation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepoCategory {
    pub category: String,
    pub confidence: f64,
    pub source: String,
    pub evidence: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepoRadarSnapshot {
    pub maturity_band: String,
    pub radar_relevance: f64,
    pub trend_signal: f64,
    pub explanation: Value,
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
    pub vitality_inputs: VitalityInputs,
    pub recent_signals: Vec<RepoSignal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_snapshot: Option<ScoreSnapshot>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VitalityInputs {
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub distinct_contributors_90d: Option<i32>,
    pub commits_30d: Option<i32>,
    pub has_ci: Option<bool>,
    pub releases_count: Option<i32>,
    pub last_release_at: Option<DateTime<Utc>>,
    pub owner_last_activity_at: Option<DateTime<Utc>>,
    pub owner_inactive_days: Option<i32>,
}
