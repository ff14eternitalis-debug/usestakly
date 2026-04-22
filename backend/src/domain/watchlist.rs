use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchedRepo {
    pub id: Uuid,
    pub artifact_id: Uuid,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub language: Option<String>,
    pub stars_count: i32,
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub muted: bool,
    pub watched_at: DateTime<Utc>,
    pub overall: Option<f64>,
    pub abandonment: Option<f64>,
    pub flags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWatchRequest {
    pub external_artifact_id: Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_kind", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationKind {
    ScoreDrop,
    AbandonmentUp,
    FlagAdded,
    FlagSevere,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Uuid,
    pub artifact_id: Uuid,
    pub owner: Option<String>,
    pub name: Option<String>,
    pub kind: NotificationKind,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}
