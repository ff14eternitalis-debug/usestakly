use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SnippetRecord {
    pub id: Uuid,
    pub library_id: Uuid,
    pub owner_id: Uuid,
    pub slug: String,
    pub domain: String,
    pub kind: String,
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    pub language: String,
    pub runtime: Option<String>,
    pub framework: Option<String>,
    pub framework_version: Option<String>,
    pub visibility: String,
    pub trust_level: String,
    pub license: String,
    pub current_version_id: Option<Uuid>,
    pub rule_set_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SnippetVersionRecord {
    pub id: Uuid,
    pub snippet_id: Uuid,
    pub version: String,
    pub code: String,
    pub variables: Value,
    pub css_classes: Option<Vec<String>>,
    pub dependencies: Value,
    pub exports: Value,
    pub imports: Value,
    pub compatibility: Value,
    pub metadata: Value,
    pub content_hash: String,
    pub risk_level: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetDetail {
    pub snippet: SnippetRecord,
    pub current_version: Option<SnippetVersionRecord>,
    pub tags: Vec<String>,
    pub canonical_reference: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippetRequest {
    pub library_id: Uuid,
    #[validate(length(min = 3, max = 180))]
    pub slug: String,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(min = 1, max = 32))]
    pub domain: String,
    #[validate(length(min = 1, max = 64))]
    pub kind: String,
    #[validate(length(min = 1, max = 80))]
    pub category: String,
    #[validate(length(min = 1, max = 64))]
    pub language: String,
    #[validate(length(max = 64))]
    pub runtime: Option<String>,
    #[validate(length(max = 64))]
    pub framework: Option<String>,
    #[validate(length(max = 64))]
    pub framework_version: Option<String>,
    pub visibility: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    #[validate(length(max = 64))]
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
    pub initial_version: CreateSnippetVersionRequest,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnippetRequest {
    #[validate(length(min = 3, max = 180))]
    pub slug: Option<String>,
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 80))]
    pub category: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    #[validate(length(max = 64))]
    pub runtime: Option<String>,
    #[validate(length(max = 64))]
    pub framework: Option<String>,
    #[validate(length(max = 64))]
    pub framework_version: Option<String>,
    pub visibility: Option<String>,
    #[validate(length(max = 64))]
    pub license: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSnippetVersionRequest {
    #[validate(length(min = 1, max = 32))]
    pub version: String,
    #[validate(length(min = 1, max = 200_000))]
    pub code: String,
    pub variables: Option<Value>,
    pub css_classes: Option<Vec<String>>,
    pub dependencies: Option<Value>,
    pub exports: Option<Value>,
    pub imports: Option<Value>,
    pub compatibility: Option<Value>,
    pub metadata: Option<Value>,
    #[validate(length(max = 32))]
    pub risk_level: Option<String>,
}
