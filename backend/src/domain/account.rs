use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReputationSummary {
    pub user_id: Uuid,
    pub score: f64,
    pub tier: String,
    pub account_age_days: i64,
    pub passive_signal_count: i64,
    pub resolve_count: i64,
    pub re_resolve_count: i64,
    pub build_success_count: i64,
    pub build_failure_count: i64,
    pub regret_count: i64,
    pub usage_signal_count: i64,
    pub successful_outcome_ratio: f64,
    pub build_reliability_ratio: f64,
    pub regret_ratio: f64,
    pub active_signal_eligible: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    pub reputation: UserReputationSummary,
    pub active_signal_min_reputation: f64,
    pub active_signal_default_consensus: u32,
    pub active_signal_severe_consensus: u32,
}
