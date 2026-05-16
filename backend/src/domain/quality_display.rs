use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofTier {
    CorpusOnly,
    UsageLimited,
    CommunityBacked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionState {
    pub key: String,
    pub value: Option<f64>,
    pub display_state: String,
    pub source: String,
    pub confidence: String,
    pub as_of: DateTime<Utc>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestionStatus {
    pub priors_fetched_at: Option<DateTime<Utc>>,
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub structural_stale: bool,
    pub structural_complete: bool,
    pub partial_fields: Vec<String>,
}
