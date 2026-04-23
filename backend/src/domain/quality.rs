use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Snippet,
    External,
}

impl ArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactKind::Snippet => "snippet",
            ArtifactKind::External => "external",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    Resolve,
    BuildSuccess,
    BuildFailure,
    Regret,
    ReResolve,
    WorksInProd,
    Broken,
    SecurityIssue,
    Deprecated,
    DoesntMatchClaim,
}

impl SignalKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SignalKind::Resolve => "resolve",
            SignalKind::BuildSuccess => "build_success",
            SignalKind::BuildFailure => "build_failure",
            SignalKind::Regret => "regret",
            SignalKind::ReResolve => "re_resolve",
            SignalKind::WorksInProd => "works_in_prod",
            SignalKind::Broken => "broken",
            SignalKind::SecurityIssue => "security_issue",
            SignalKind::Deprecated => "deprecated",
            SignalKind::DoesntMatchClaim => "doesnt_match_claim",
        }
    }

    pub fn is_passive(self) -> bool {
        matches!(
            self,
            SignalKind::Resolve
                | SignalKind::BuildSuccess
                | SignalKind::BuildFailure
                | SignalKind::Regret
                | SignalKind::ReResolve
        )
    }

    pub fn requires_evidence(self) -> bool {
        matches!(
            self,
            SignalKind::Broken
                | SignalKind::SecurityIssue
                | SignalKind::DoesntMatchClaim
                | SignalKind::Deprecated
        )
    }
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QualitySignalRecord {
    pub id: Uuid,
    pub artifact_kind: String,
    pub snippet_id: Option<Uuid>,
    pub external_artifact_id: Option<Uuid>,
    pub signal: String,
    pub is_passive: bool,
    pub actor_user_id: Option<Uuid>,
    pub agent_context: Option<Value>,
    pub evidence_url: Option<String>,
    pub evidence_description: Option<String>,
    pub review_status: String,
    pub reviewed_by_user_id: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_note: Option<String>,
    pub disputed_by_user_id: Option<Uuid>,
    pub disputed_at: Option<DateTime<Utc>>,
    pub dispute_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSignalRequest {
    pub signal: SignalKind,
    #[validate(url)]
    pub evidence_url: Option<String>,
    #[validate(length(max = 2000))]
    pub evidence_description: Option<String>,
    pub agent_context: Option<Value>,
}
