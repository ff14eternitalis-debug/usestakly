use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LibraryRecord {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub visibility: String,
    pub trust_level: String,
    pub is_default: bool,
    pub default_stack: Value,
    pub allowed_domains: Value,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateLibraryRequest {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(min = 3, max = 160))]
    pub slug: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub is_default: Option<bool>,
    pub default_stack: Option<Value>,
    pub allowed_domains: Option<Value>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLibraryRequest {
    #[validate(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 160))]
    pub slug: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub is_default: Option<bool>,
    pub default_stack: Option<Value>,
    pub allowed_domains: Option<Value>,
    pub metadata: Option<Value>,
}
