mod crypto;
mod discord;
mod email;
mod message;
mod model;
mod store;

pub use model::{
    NotificationChannelSummary, NotificationChannelType, UpsertNotificationChannel,
    WatchAlertDelivery,
};

pub(crate) use crypto::decrypt_webhook_url;
pub use discord::{mask_discord_webhook_url, validate_discord_webhook_url};
pub(crate) use email::send_email;
pub use email::validate_notification_email;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig};

use model::{ChannelRow, ChannelSecretRow, DeliveryChannelRow};

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
            let email = email::validate_notification_email(email)?;
            (email, None)
        }
        NotificationChannelType::DiscordWebhook => {
            if let Some(url) = input
                .webhook_url
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                let url = discord::validate_discord_webhook_url(url)?;
                let masked = discord::mask_discord_webhook_url(&url);
                let encrypted = crypto::encrypt_webhook_url(secret, &url)
                    .map_err(|_| ApiError::internal("failed to encrypt webhook URL"))?;
                (masked, Some(encrypted))
            } else {
                let existing =
                    store::existing_channel_secret(db, user_id, "discord_webhook").await?;
                (existing.destination, existing.secret_ciphertext)
            }
        }
    };
    if let Some(locale) = input.email_locale.as_deref() {
        let email_locale = crate::services::account_preferences::validate_email_locale(locale)?;
        sqlx::query("UPDATE users SET email_locale = $2, updated_at = NOW() WHERE id = $1")
            .bind(user_id)
            .bind(email_locale)
            .execute(db)
            .await?;
    }

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
        NotificationChannelType::Email => email::post_email_test_message(config, &row.destination)
            .await
            .map_err(|err| ApiError::bad_request(err.to_string())),
        NotificationChannelType::DiscordWebhook => {
            let ciphertext = row
                .secret_ciphertext
                .ok_or_else(|| ApiError::internal("missing webhook secret"))?;
            let webhook_url = crypto::decrypt_webhook_url(secret, &ciphertext)
                .map_err(|_| ApiError::internal("failed to decrypt webhook URL"))?;
            discord::post_discord_test_message(&webhook_url)
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
        SELECT c.id, c.channel_type, c.destination, c.secret_ciphertext
        FROM notification_channels c
        JOIN users u ON u.id = c.user_id
        WHERE c.user_id = $1
          AND c.channel_type IN ('discord_webhook', 'email')
          AND c.enabled = TRUE
          AND c.critical_alerts_enabled = TRUE
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let mut delivered = 0usize;
    for row in rows {
        match NotificationChannelType::from_db(row.channel_type.clone())? {
            NotificationChannelType::Email => {
                match email::post_email_watch_alert(
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
                        store::update_delivery_error(db, row.id, &err.to_string()).await?;
                    }
                }
            }
            NotificationChannelType::DiscordWebhook => {
                let Some(ciphertext) = row.secret_ciphertext else {
                    continue;
                };
                let webhook_url = match crypto::decrypt_webhook_url(delivery.secret, &ciphertext) {
                    Ok(url) => url,
                    Err(err) => {
                        store::update_delivery_error(
                            db,
                            row.id,
                            &format!("failed to decrypt webhook URL: {err}"),
                        )
                        .await?;
                        continue;
                    }
                };

                match discord::post_discord_watch_alert(
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
                        store::update_delivery_error(db, row.id, &err.to_string()).await?;
                    }
                }
            }
        }
    }

    Ok(delivered)
}
