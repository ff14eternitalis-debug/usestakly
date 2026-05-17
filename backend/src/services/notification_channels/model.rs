use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig, domain::watchlist::NotificationKind};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannelType {
    Email,
    DiscordWebhook,
}

impl NotificationChannelType {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::DiscordWebhook => "discord_webhook",
        }
    }

    pub(crate) fn from_db(value: String) -> Result<Self, ApiError> {
        match value.as_str() {
            "email" => Ok(Self::Email),
            "discord_webhook" => Ok(Self::DiscordWebhook),
            _ => Err(ApiError::internal("unknown notification channel type")),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationChannelSummary {
    pub id: Uuid,
    pub channel_type: NotificationChannelType,
    pub label: String,
    pub destination: String,
    pub enabled: bool,
    pub critical_alerts_enabled: bool,
    pub daily_digest_enabled: bool,
    pub last_tested_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertNotificationChannel {
    pub channel_type: NotificationChannelType,
    pub label: Option<String>,
    pub email: Option<String>,
    pub webhook_url: Option<String>,
    pub enabled: Option<bool>,
    pub critical_alerts_enabled: Option<bool>,
    pub daily_digest_enabled: Option<bool>,
    pub email_locale: Option<String>,
}

#[derive(FromRow)]
pub(crate) struct ChannelRow {
    pub id: Uuid,
    pub channel_type: String,
    pub label: String,
    pub destination: String,
    pub enabled: bool,
    pub critical_alerts_enabled: bool,
    pub daily_digest_enabled: bool,
    pub last_tested_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub(crate) struct ChannelSecretRow {
    pub id: Uuid,
    pub channel_type: String,
    pub destination: String,
    pub secret_ciphertext: Option<String>,
}

#[derive(FromRow)]
pub(crate) struct ExistingChannelSecretRow {
    pub destination: String,
    pub secret_ciphertext: Option<String>,
}

#[derive(FromRow)]
pub(crate) struct DeliveryChannelRow {
    pub id: Uuid,
    pub channel_type: String,
    pub destination: String,
    pub secret_ciphertext: Option<String>,
}

pub struct WatchAlertDelivery<'a> {
    pub secret: &'a str,
    pub config: &'a AppConfig,
    pub repo_full_name: &'a str,
    pub repo_url: Option<&'a str>,
    pub kind: NotificationKind,
    pub payload: &'a serde_json::Value,
}

impl TryFrom<ChannelRow> for NotificationChannelSummary {
    type Error = ApiError;

    fn try_from(row: ChannelRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            channel_type: NotificationChannelType::from_db(row.channel_type)?,
            label: row.label,
            destination: row.destination,
            enabled: row.enabled,
            critical_alerts_enabled: row.critical_alerts_enabled,
            daily_digest_enabled: row.daily_digest_enabled,
            last_tested_at: row.last_tested_at,
            last_error: row.last_error,
            created_at: row.created_at,
        })
    }
}
