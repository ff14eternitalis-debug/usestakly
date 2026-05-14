use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Utc};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    message::{MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use validator::ValidateEmail;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::watchlist::NotificationKind,
    services::email_templates::{
        EmailField, EmailTemplate, render_test_email, render_watch_alert_email,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannelType {
    Email,
    DiscordWebhook,
}

impl NotificationChannelType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::DiscordWebhook => "discord_webhook",
        }
    }

    fn from_db(value: String) -> Result<Self, ApiError> {
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
}

#[derive(FromRow)]
struct ChannelRow {
    id: Uuid,
    channel_type: String,
    label: String,
    destination: String,
    enabled: bool,
    critical_alerts_enabled: bool,
    daily_digest_enabled: bool,
    last_tested_at: Option<DateTime<Utc>>,
    last_error: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(FromRow)]
struct ChannelSecretRow {
    id: Uuid,
    channel_type: String,
    destination: String,
    secret_ciphertext: Option<String>,
}

#[derive(FromRow)]
struct ExistingChannelSecretRow {
    destination: String,
    secret_ciphertext: Option<String>,
}

#[derive(FromRow)]
struct DeliveryChannelRow {
    id: Uuid,
    channel_type: String,
    destination: String,
    secret_ciphertext: Option<String>,
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

pub async fn list_for_user(
    db: &PgPool,
    user_id: Uuid,
) -> Result<Vec<NotificationChannelSummary>, ApiError> {
    let rows: Vec<ChannelRow> = sqlx::query_as(
        r#"
        SELECT id, channel_type, label, destination, enabled,
               critical_alerts_enabled, daily_digest_enabled,
               last_tested_at, last_error, created_at
        FROM notification_channels
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    rows.into_iter()
        .map(NotificationChannelSummary::try_from)
        .collect()
}

pub async fn upsert(
    db: &PgPool,
    user_id: Uuid,
    secret: &str,
    input: UpsertNotificationChannel,
) -> Result<NotificationChannelSummary, ApiError> {
    let label = input
        .label
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(match input.channel_type {
            NotificationChannelType::Email => "Email",
            NotificationChannelType::DiscordWebhook => "Discord",
        });
    if label.len() > 80 {
        return Err(ApiError::bad_request("label too long (max 80)"));
    }

    let (destination, secret_ciphertext) = match input.channel_type {
        NotificationChannelType::Email => {
            let email = input
                .email
                .as_deref()
                .ok_or_else(|| ApiError::bad_request("email is required"))?;
            let email = validate_notification_email(email)?;
            (email, None)
        }
        NotificationChannelType::DiscordWebhook => {
            if let Some(url) = input
                .webhook_url
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                let url = validate_discord_webhook_url(url)?;
                let masked = mask_discord_webhook_url(&url);
                let encrypted = encrypt_webhook_url(secret, &url)
                    .map_err(|_| ApiError::internal("failed to encrypt webhook URL"))?;
                (masked, Some(encrypted))
            } else {
                let existing = existing_channel_secret(db, user_id, "discord_webhook").await?;
                (existing.destination, existing.secret_ciphertext)
            }
        }
    };

    let row: ChannelRow = sqlx::query_as(
        r#"
        INSERT INTO notification_channels (
          user_id, channel_type, label, destination, secret_ciphertext,
          enabled, critical_alerts_enabled, daily_digest_enabled
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (user_id, channel_type)
        DO UPDATE SET
          label = EXCLUDED.label,
          destination = EXCLUDED.destination,
          secret_ciphertext = EXCLUDED.secret_ciphertext,
          enabled = EXCLUDED.enabled,
          critical_alerts_enabled = EXCLUDED.critical_alerts_enabled,
          daily_digest_enabled = EXCLUDED.daily_digest_enabled,
          last_error = NULL,
          updated_at = NOW()
        RETURNING id, channel_type, label, destination, enabled,
                  critical_alerts_enabled, daily_digest_enabled,
                  last_tested_at, last_error, created_at
        "#,
    )
    .bind(user_id)
    .bind(input.channel_type.as_str())
    .bind(label)
    .bind(destination)
    .bind(secret_ciphertext)
    .bind(input.enabled.unwrap_or(true))
    .bind(input.critical_alerts_enabled.unwrap_or(true))
    .bind(input.daily_digest_enabled.unwrap_or(false))
    .fetch_one(db)
    .await?;

    row.try_into()
}

async fn existing_channel_secret(
    db: &PgPool,
    user_id: Uuid,
    channel_type: &str,
) -> Result<ExistingChannelSecretRow, ApiError> {
    sqlx::query_as(
        r#"
        SELECT destination, secret_ciphertext
        FROM notification_channels
        WHERE user_id = $1 AND channel_type = $2
        "#,
    )
    .bind(user_id)
    .bind(channel_type)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::bad_request("webhookUrl is required"))
}

pub async fn delete(db: &PgPool, user_id: Uuid, channel_id: Uuid) -> Result<(), ApiError> {
    let rows = sqlx::query("DELETE FROM notification_channels WHERE id = $1 AND user_id = $2")
        .bind(channel_id)
        .bind(user_id)
        .execute(db)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(ApiError::not_found("Notification channel not found"));
    }
    Ok(())
}

pub async fn send_test(
    db: &PgPool,
    user_id: Uuid,
    secret: &str,
    config: &AppConfig,
    channel_id: Uuid,
) -> Result<(), ApiError> {
    let row: ChannelSecretRow = sqlx::query_as(
        r#"
        SELECT id, channel_type, destination, secret_ciphertext
        FROM notification_channels
        WHERE id = $1 AND user_id = $2 AND enabled = TRUE
        "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Notification channel not found"))?;

    let result = match NotificationChannelType::from_db(row.channel_type)? {
        NotificationChannelType::Email => post_email_test_message(config, &row.destination)
            .await
            .map_err(|err| ApiError::bad_request(err.to_string())),
        NotificationChannelType::DiscordWebhook => {
            let ciphertext = row
                .secret_ciphertext
                .ok_or_else(|| ApiError::internal("missing webhook secret"))?;
            let webhook_url = decrypt_webhook_url(secret, &ciphertext)
                .map_err(|_| ApiError::internal("failed to decrypt webhook URL"))?;
            post_discord_test_message(&webhook_url)
                .await
                .map_err(|err| ApiError::bad_request(err.to_string()))
        }
    };

    match result {
        Ok(()) => {
            sqlx::query(
                "UPDATE notification_channels SET last_tested_at = NOW(), last_error = NULL, updated_at = NOW() WHERE id = $1",
            )
            .bind(row.id)
            .execute(db)
            .await?;
            Ok(())
        }
        Err(err) => {
            sqlx::query(
                "UPDATE notification_channels SET last_error = $2, updated_at = NOW() WHERE id = $1",
            )
            .bind(row.id)
            .bind(err.message.chars().take(240).collect::<String>())
            .execute(db)
            .await?;
            Err(err)
        }
    }
}

pub async fn deliver_watch_alert(
    db: &PgPool,
    user_id: Uuid,
    delivery: WatchAlertDelivery<'_>,
) -> Result<usize, ApiError> {
    let rows: Vec<DeliveryChannelRow> = sqlx::query_as(
        r#"
        SELECT id, channel_type, destination, secret_ciphertext
        FROM notification_channels
        WHERE user_id = $1
          AND channel_type IN ('discord_webhook', 'email')
          AND enabled = TRUE
          AND critical_alerts_enabled = TRUE
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let mut delivered = 0usize;
    for row in rows {
        match NotificationChannelType::from_db(row.channel_type.clone())? {
            NotificationChannelType::Email => {
                match post_email_watch_alert(
                    delivery.config,
                    &row.destination,
                    delivery.repo_full_name,
                    delivery.repo_url,
                    delivery.kind,
                    delivery.payload,
                )
                .await
                {
                    Ok(()) => {
                        sqlx::query(
                            "UPDATE notification_channels SET last_error = NULL, updated_at = NOW() WHERE id = $1",
                        )
                        .bind(row.id)
                        .execute(db)
                        .await?;
                        delivered += 1;
                    }
                    Err(err) => {
                        update_delivery_error(db, row.id, &err.to_string()).await?;
                    }
                }
            }
            NotificationChannelType::DiscordWebhook => {
                let Some(ciphertext) = row.secret_ciphertext else {
                    continue;
                };
                let webhook_url = match decrypt_webhook_url(delivery.secret, &ciphertext) {
                    Ok(url) => url,
                    Err(err) => {
                        update_delivery_error(
                            db,
                            row.id,
                            &format!("failed to decrypt webhook URL: {err}"),
                        )
                        .await?;
                        continue;
                    }
                };

                match post_discord_watch_alert(
                    &webhook_url,
                    delivery.repo_full_name,
                    delivery.repo_url,
                    delivery.kind,
                    delivery.payload,
                )
                .await
                {
                    Ok(()) => {
                        sqlx::query(
                            "UPDATE notification_channels SET last_error = NULL, updated_at = NOW() WHERE id = $1",
                        )
                        .bind(row.id)
                        .execute(db)
                        .await?;
                        delivered += 1;
                    }
                    Err(err) => {
                        update_delivery_error(db, row.id, &err.to_string()).await?;
                    }
                }
            }
        }
    }

    Ok(delivered)
}

pub fn validate_notification_email(value: &str) -> Result<String, ApiError> {
    let email = value.trim().to_ascii_lowercase();
    if !email.validate_email()
        || !email
            .split('@')
            .nth(1)
            .is_some_and(|domain| domain.contains('.'))
    {
        return Err(ApiError::bad_request("invalid email address"));
    }
    Ok(email)
}

pub fn validate_discord_webhook_url(value: &str) -> Result<String, ApiError> {
    let trimmed = value.trim();
    let parsed = Url::parse(trimmed).map_err(|_| ApiError::bad_request("invalid webhook URL"))?;
    if parsed.scheme() != "https" {
        return Err(ApiError::bad_request("webhook URL must use HTTPS"));
    }
    let host = parsed.host_str().unwrap_or_default();
    if host != "discord.com" && host != "discordapp.com" {
        return Err(ApiError::bad_request("only Discord webhooks are supported"));
    }
    let segments: Vec<_> = parsed
        .path_segments()
        .map(|segments| segments.collect())
        .unwrap_or_default();
    if segments.len() < 4
        || segments[0] != "api"
        || segments[1] != "webhooks"
        || segments[2].is_empty()
        || segments[3].is_empty()
    {
        return Err(ApiError::bad_request("invalid Discord webhook URL"));
    }
    Ok(trimmed.to_string())
}

pub fn mask_discord_webhook_url(value: &str) -> String {
    let parsed = match Url::parse(value) {
        Ok(parsed) => parsed,
        Err(_) => return "discord webhook ...".to_string(),
    };
    let segments: Vec<_> = parsed
        .path_segments()
        .map(|segments| segments.collect())
        .unwrap_or_default();
    let id = segments.get(2).copied().unwrap_or("unknown");
    let token = segments.get(3).copied().unwrap_or("");
    let tail = token
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("discord webhook {id}/...{tail}")
}

pub fn encrypt_webhook_url(secret: &str, plaintext: &str) -> Result<String, String> {
    let cipher = cipher_from_secret(secret)?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| "encrypt failed".to_string())?;
    let mut payload = nonce_bytes.to_vec();
    payload.extend(ciphertext);
    Ok(general_purpose::STANDARD.encode(payload))
}

pub fn decrypt_webhook_url(secret: &str, ciphertext: &str) -> Result<String, String> {
    let payload = general_purpose::STANDARD
        .decode(ciphertext)
        .map_err(|_| "invalid base64".to_string())?;
    if payload.len() <= 12 {
        return Err("invalid ciphertext".to_string());
    }
    let (nonce_bytes, encrypted) = payload.split_at(12);
    let cipher = cipher_from_secret(secret)?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), encrypted)
        .map_err(|_| "decrypt failed".to_string())?;
    String::from_utf8(plaintext).map_err(|_| "invalid utf8".to_string())
}

fn cipher_from_secret(secret: &str) -> Result<Aes256Gcm, String> {
    if secret.len() < 16 {
        return Err("secret too short".to_string());
    }
    let key = Sha256::digest(secret.as_bytes());
    Aes256Gcm::new_from_slice(&key).map_err(|_| "invalid key".to_string())
}

async fn post_discord_test_message(webhook_url: &str) -> Result<(), anyhow::Error> {
    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&json!({
            "username": "UseStakly",
            "content": "UseStakly notification test.",
            "allowed_mentions": { "parse": [] },
            "embeds": [{
                "title": "Notification channel connected",
                "description": "UseStakly can now send critical watch alerts to this Discord channel.",
                "color": 8900331
            }]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Discord webhook returned HTTP {}", response.status());
    }
    Ok(())
}

async fn post_email_test_message(config: &AppConfig, to: &str) -> Result<(), anyhow::Error> {
    send_email(config, to, &render_test_email()).await
}

async fn post_discord_watch_alert(
    webhook_url: &str,
    repo_full_name: &str,
    repo_url: Option<&str>,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    let message = watch_alert_message(repo_full_name, kind, payload);
    let mut embed = json!({
        "title": message.title,
        "description": message.description,
        "color": message.color,
        "fields": message.fields
    });
    if let Some(url) = repo_url {
        embed["url"] = json!(url);
    }

    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&json!({
            "username": "UseStakly",
            "content": message.content,
            "allowed_mentions": { "parse": [] },
            "embeds": [embed]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Discord webhook returned HTTP {}", response.status());
    }
    Ok(())
}

async fn post_email_watch_alert(
    config: &AppConfig,
    to: &str,
    repo_full_name: &str,
    repo_url: Option<&str>,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    let message = watch_alert_message(repo_full_name, kind, payload);
    let fields = message
        .fields
        .iter()
        .filter_map(|field| {
            let name = field
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("Detail");
            let value = field
                .get("value")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            (!value.is_empty()).then(|| EmailField {
                name: name.to_string(),
                value: value.to_string(),
            })
        })
        .collect::<Vec<_>>();
    let email = render_watch_alert_email(
        &format!("[UseStakly] {}", message.title),
        &message.title,
        &message.content,
        &message.description,
        repo_url,
        &fields,
    );

    send_email(config, to, &email).await
}

pub(crate) async fn send_email(
    config: &AppConfig,
    to: &str,
    email: &EmailTemplate,
) -> Result<(), anyhow::Error> {
    let (Some(username), Some(password)) = (
        config.email_smtp_username.as_deref(),
        config.email_smtp_password.as_deref(),
    ) else {
        anyhow::bail!("email SMTP is not configured");
    };

    let from: Mailbox = Mailbox::new(
        Some(config.email_from_name.clone()),
        config.email_from_address.parse()?,
    );
    let to: Mailbox = to.parse()?;
    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(&email.subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(email.text.clone()))
                .singlepart(SinglePart::html(email.html.clone())),
        )?;
    let creds = Credentials::new(username.to_string(), password.to_string());
    let transport = if config.email_smtp_port == 465 {
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.email_smtp_host)?
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.email_smtp_host)?
    };
    let mailer = transport
        .port(config.email_smtp_port)
        .credentials(creds)
        .build();

    mailer.send(email).await?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct WatchAlertMessage {
    title: String,
    content: String,
    description: String,
    color: u32,
    fields: Vec<serde_json::Value>,
}

fn watch_alert_message(
    repo_full_name: &str,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> WatchAlertMessage {
    match kind {
        NotificationKind::ScoreDrop => {
            let prev = payload
                .get("prev_overall")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or_default();
            let new = payload
                .get("new_overall")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or_default();
            WatchAlertMessage {
                title: format!("{repo_full_name}: score drop"),
                content: format!("UseStakly alert: {repo_full_name} quality score dropped."),
                description: "A watched repository crossed the score-drop alert threshold."
                    .to_string(),
                color: 16_744_996,
                fields: vec![
                    json!({ "name": "Previous score", "value": format!("{prev:.2}"), "inline": true }),
                    json!({ "name": "New score", "value": format!("{new:.2}"), "inline": true }),
                ],
            }
        }
        NotificationKind::AbandonmentUp => WatchAlertMessage {
            title: format!("{repo_full_name}: abandonment risk up"),
            content: format!("UseStakly alert: {repo_full_name} abandonment risk increased."),
            description: "A watched repository shows a higher abandonment risk.".to_string(),
            color: 16_744_996,
            fields: vec![],
        },
        NotificationKind::FlagAdded | NotificationKind::FlagSevere => {
            let flag = payload
                .get("flag")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("new flag");
            let severe = matches!(kind, NotificationKind::FlagSevere);
            WatchAlertMessage {
                title: format!("{repo_full_name}: {flag}"),
                content: format!("UseStakly alert: {repo_full_name} received flag `{flag}`."),
                description: if severe {
                    "A severe flag was detected on a watched repository."
                } else {
                    "A new flag was detected on a watched repository."
                }
                .to_string(),
                color: if severe { 15_115_908 } else { 16_744_996 },
                fields: vec![json!({ "name": "Flag", "value": flag, "inline": true })],
            }
        }
        NotificationKind::UseCaseNewCandidate => WatchAlertMessage {
            title: format!("{repo_full_name}: new radar candidate"),
            content: format!("UseStakly alert: {repo_full_name} entered a watched need."),
            description: "A repository entered the recommendations for a watched need.".to_string(),
            color: 8_900_331,
            fields: vec![],
        },
        NotificationKind::UseCaseBestCandidateChanged => WatchAlertMessage {
            title: format!("{repo_full_name}: best radar candidate changed"),
            content: format!(
                "UseStakly alert: {repo_full_name} is now the top match for a watched need."
            ),
            description: "The leading recommendation changed for a watched need.".to_string(),
            color: 8_900_331,
            fields: vec![],
        },
        NotificationKind::UseCaseQualityDrop => WatchAlertMessage {
            title: format!("{repo_full_name}: radar candidate quality dropped"),
            content: format!("UseStakly alert: {repo_full_name} dropped in a watched need."),
            description: "A repository in a watched need crossed the quality-drop threshold."
                .to_string(),
            color: 16_744_996,
            fields: vec![],
        },
        NotificationKind::UseCaseFlagAdded => {
            let flag = payload
                .get("flag")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("new flag");
            WatchAlertMessage {
                title: format!("{repo_full_name}: radar candidate flag"),
                content: format!(
                    "UseStakly alert: {repo_full_name} received flag `{flag}` in a watched need."
                ),
                description: "A repository in a watched need received a new flag.".to_string(),
                color: 16_744_996,
                fields: vec![json!({ "name": "Flag", "value": flag, "inline": true })],
            }
        }
    }
}

async fn update_delivery_error(
    db: &PgPool,
    channel_id: Uuid,
    message: &str,
) -> Result<(), ApiError> {
    sqlx::query(
        "UPDATE notification_channels SET last_error = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(channel_id)
    .bind(message.chars().take(240).collect::<String>())
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_email_rejects_invalid_address() {
        assert!(validate_notification_email("dev@example.com").is_ok());
        assert!(validate_notification_email("not-an-email").is_err());
        assert!(validate_notification_email("dev@localhost").is_err());
    }

    #[test]
    fn discord_webhook_accepts_only_discord_webhook_urls() {
        assert!(
            validate_discord_webhook_url(
                "https://discord.com/api/webhooks/123456789012345678/token-value"
            )
            .is_ok()
        );
        assert!(
            validate_discord_webhook_url(
                "https://discordapp.com/api/webhooks/123456789012345678/token-value"
            )
            .is_ok()
        );
        assert!(validate_discord_webhook_url("https://example.com/webhook").is_err());
    }

    #[test]
    fn discord_webhook_url_is_masked_without_leaking_secret() {
        let masked = mask_discord_webhook_url(
            "https://discord.com/api/webhooks/123456789012345678/abcdefghijklmnopqrstuvwxyz",
        );

        assert_eq!(masked, "discord webhook 123456789012345678/...wxyz");
    }

    #[test]
    fn webhook_url_roundtrips_through_encryption() {
        let secret = "test-session-secret-long-enough";
        let plaintext =
            "https://discord.com/api/webhooks/123456789012345678/abcdefghijklmnopqrstuvwxyz";

        let encrypted = encrypt_webhook_url(secret, plaintext).expect("encrypt");
        assert_ne!(encrypted, plaintext);
        assert!(!encrypted.contains("abcdefghijklmnopqrstuvwxyz"));

        let decrypted = decrypt_webhook_url(secret, &encrypted).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn watch_alert_message_explains_score_drop() {
        let message = watch_alert_message(
            "facebook/react",
            NotificationKind::ScoreDrop,
            &json!({
                "prev_overall": 0.84,
                "new_overall": 0.68,
            }),
        );

        assert_eq!(message.title, "facebook/react: score drop");
        assert!(message.content.contains("facebook/react"));
        assert!(message.description.contains("score-drop alert threshold"));
        assert_eq!(message.fields.len(), 2);
    }
}
