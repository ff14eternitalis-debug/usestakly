use chrono::{DateTime, NaiveDate, Timelike, Utc};
use chrono_tz::Tz;
use serde_json::json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    services::{
        email_templates::{EmailSection, render_digest_email},
        notification_channels::{decrypt_webhook_url, send_email},
    },
};

pub fn digest_time_for_preset(preset: &str) -> Result<&'static str, ApiError> {
    match preset {
        "morning" => Ok("08:00"),
        "noon" => Ok("12:00"),
        "evening" => Ok("18:00"),
        "night" => Ok("21:00"),
        _ => Err(ApiError::bad_request("invalid digest time preset")),
    }
}

pub fn is_digest_due_now(
    digest_time_local: &str,
    timezone: &str,
    now: DateTime<Utc>,
    window_minutes: i64,
) -> Result<bool, ApiError> {
    let scheduled_minutes = parse_hhmm_minutes(digest_time_local)?;
    let tz: Tz = timezone
        .parse()
        .map_err(|_| ApiError::bad_request("invalid timezone"))?;
    let local_now = now.with_timezone(&tz);
    let now_minutes = i64::from(local_now.hour()) * 60 + i64::from(local_now.minute());
    let diff = now_minutes - scheduled_minutes;
    Ok((0..window_minutes).contains(&diff))
}

fn digest_date_for_timezone(timezone: &str, now: DateTime<Utc>) -> Result<NaiveDate, ApiError> {
    let tz: Tz = timezone
        .parse()
        .map_err(|_| ApiError::bad_request("invalid timezone"))?;
    Ok(now.with_timezone(&tz).date_naive())
}

fn parse_hhmm_minutes(value: &str) -> Result<i64, ApiError> {
    let (hours, minutes) = value
        .split_once(':')
        .ok_or_else(|| ApiError::bad_request("invalid digest time"))?;
    let hours: i64 = hours
        .parse()
        .map_err(|_| ApiError::bad_request("invalid digest time"))?;
    let minutes: i64 = minutes
        .parse()
        .map_err(|_| ApiError::bad_request("invalid digest time"))?;
    if !(0..=23).contains(&hours) || !(0..=59).contains(&minutes) {
        return Err(ApiError::bad_request("invalid digest time"));
    }
    Ok(hours * 60 + minutes)
}

#[derive(Debug, FromRow)]
struct DueDigestChannelRow {
    user_id: Uuid,
    channel_id: Uuid,
    channel_type: String,
    destination: String,
    secret_ciphertext: Option<String>,
    digest_time_local: String,
    timezone: String,
}

#[derive(Debug, FromRow)]
struct NotificationDigestRow {
    owner: Option<String>,
    name: Option<String>,
    kind: String,
}

#[derive(Debug, FromRow)]
struct RadarCandidateRow {
    owner: Option<String>,
    name: Option<String>,
}

#[derive(Debug)]
struct DigestContent {
    score_drops: Vec<String>,
    abandonment_up: Vec<String>,
    new_flags: Vec<String>,
    radar_candidates: Vec<String>,
}

impl DigestContent {
    fn is_empty(&self) -> bool {
        self.score_drops.is_empty()
            && self.abandonment_up.is_empty()
            && self.new_flags.is_empty()
            && self.radar_candidates.is_empty()
    }
}

pub async fn run_due_digests(
    db: &PgPool,
    config: &AppConfig,
    now: DateTime<Utc>,
    window_minutes: i64,
) -> Result<usize, ApiError> {
    let rows: Vec<DueDigestChannelRow> = sqlx::query_as(
        r#"
        SELECT
          c.user_id,
          c.id AS channel_id,
          c.channel_type,
          c.destination,
          c.secret_ciphertext,
          u.digest_time_local,
          u.timezone
        FROM notification_channels c
        JOIN users u ON u.id = c.user_id
        WHERE c.enabled = TRUE
          AND c.daily_digest_enabled = TRUE
          AND c.channel_type IN ('discord_webhook', 'email')
        "#,
    )
    .fetch_all(db)
    .await?;

    let mut delivered = 0usize;
    for row in rows {
        if !is_digest_due_now(&row.digest_time_local, &row.timezone, now, window_minutes)? {
            continue;
        }
        let digest_date = digest_date_for_timezone(&row.timezone, now)?;
        let Some(delivery_id) =
            reserve_digest_delivery(db, row.user_id, row.channel_id, digest_date).await?
        else {
            continue;
        };

        let content = load_digest_content(db, row.user_id).await?;
        if content.is_empty() {
            mark_digest_skipped(db, delivery_id).await?;
            continue;
        }

        let result = match row.channel_type.as_str() {
            "discord_webhook" => {
                let Some(secret) = config.notification_secret() else {
                    tracing::warn!("digest scheduler: APP_NOTIFICATION_SECRET missing");
                    mark_digest_failed(db, delivery_id, "APP_NOTIFICATION_SECRET missing").await?;
                    update_channel_error(db, row.channel_id, "APP_NOTIFICATION_SECRET missing")
                        .await?;
                    continue;
                };
                if let Some(ciphertext) = row.secret_ciphertext.as_deref() {
                    match decrypt_webhook_url(secret, ciphertext) {
                        Ok(webhook_url) => post_discord_digest(&webhook_url, &content).await,
                        Err(err) => Err(ApiError::internal(format!(
                            "failed to decrypt webhook URL: {err}"
                        ))),
                    }
                } else {
                    Err(ApiError::internal("missing webhook secret"))
                }
            }
            "email" => post_email_digest(config, &row.destination, &content).await,
            _ => Err(ApiError::internal("unknown notification channel type")),
        };

        match result {
            Ok(()) => {
                mark_digest_delivered(db, delivery_id).await?;
                delivered += 1;
            }
            Err(err) => {
                mark_digest_failed(db, delivery_id, &err.message).await?;
                update_channel_error(db, row.channel_id, &err.message).await?;
            }
        }
    }

    Ok(delivered)
}

async fn reserve_digest_delivery(
    db: &PgPool,
    user_id: Uuid,
    channel_id: Uuid,
    digest_date: NaiveDate,
) -> Result<Option<Uuid>, ApiError> {
    let id: Option<Uuid> = sqlx::query_scalar(
        r#"
        INSERT INTO notification_digest_deliveries (
          user_id, notification_channel_id, digest_date, status
        )
        VALUES ($1, $2, $3, 'pending')
        ON CONFLICT (notification_channel_id, digest_date) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(channel_id)
    .bind(digest_date)
    .fetch_optional(db)
    .await?;
    Ok(id)
}

async fn load_digest_content(db: &PgPool, user_id: Uuid) -> Result<DigestContent, ApiError> {
    let notifications: Vec<NotificationDigestRow> = sqlx::query_as(
        r#"
        SELECT e.github_owner AS owner, e.github_repo AS name, n.kind::text AS kind
        FROM notifications n
        JOIN external_artifacts e ON e.id = n.external_artifact_id
        WHERE n.user_id = $1
          AND n.created_at >= NOW() - INTERVAL '24 hours'
        ORDER BY n.created_at DESC
        LIMIT 20
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let radar_candidates: Vec<RadarCandidateRow> = sqlx::query_as(
        r#"
        SELECT DISTINCT e.github_owner AS owner, e.github_repo AS name
        FROM use_case_watch_matches m
        JOIN use_case_watches w ON w.id = m.use_case_watch_id
        JOIN external_artifacts e ON e.id = m.external_artifact_id
        WHERE w.user_id = $1
          AND w.enabled = TRUE
          AND m.last_seen_at >= NOW() - INTERVAL '24 hours'
        ORDER BY e.github_owner, e.github_repo
        LIMIT 5
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    let mut content = DigestContent {
        score_drops: Vec::new(),
        abandonment_up: Vec::new(),
        new_flags: Vec::new(),
        radar_candidates: repo_names(radar_candidates),
    };

    for notification in notifications {
        let repo = repo_name(notification.owner, notification.name);
        match notification.kind.as_str() {
            "score_drop" => push_unique(&mut content.score_drops, repo),
            "abandonment_up" => push_unique(&mut content.abandonment_up, repo),
            "flag_added" | "flag_severe" => push_unique(&mut content.new_flags, repo),
            _ => {}
        }
    }

    Ok(content)
}

async fn post_discord_digest(webhook_url: &str, content: &DigestContent) -> Result<(), ApiError> {
    let response = reqwest::Client::new()
        .post(webhook_url)
        .json(&json!({
            "username": "UseStakly",
            "content": "UseStakly daily watch digest.",
            "allowed_mentions": { "parse": [] },
            "embeds": [{
                "title": "Daily watch digest",
                "description": "Short summary of the important changes from your UseStakly watch.",
                "color": 8900331,
                "fields": digest_fields(content)
            }]
        }))
        .send()
        .await
        .map_err(|err| ApiError::bad_request(err.to_string()))?;

    if !response.status().is_success() {
        return Err(ApiError::bad_request(format!(
            "Discord webhook returned HTTP {}",
            response.status()
        )));
    }
    Ok(())
}

async fn post_email_digest(
    config: &AppConfig,
    to: &str,
    content: &DigestContent,
) -> Result<(), ApiError> {
    let email = render_digest_email(&digest_email_sections(content));
    send_email(config, to, &email)
        .await
        .map_err(|err| ApiError::bad_request(err.to_string()))
}

fn digest_email_sections(content: &DigestContent) -> Vec<EmailSection> {
    [
        ("Repos to watch", &content.abandonment_up),
        ("Scores down", &content.score_drops),
        ("New flags", &content.new_flags),
        ("New radar candidates", &content.radar_candidates),
    ]
    .into_iter()
    .filter(|(_, items)| !items.is_empty())
    .map(|(title, items)| EmailSection {
        title: title.to_string(),
        items: items.iter().take(5).cloned().collect(),
    })
    .collect()
}

fn digest_fields(content: &DigestContent) -> Vec<serde_json::Value> {
    [
        ("Repos to watch", &content.abandonment_up),
        ("Scores down", &content.score_drops),
        ("New flags", &content.new_flags),
        ("New radar candidates", &content.radar_candidates),
    ]
    .into_iter()
    .filter(|(_, items)| !items.is_empty())
    .map(|(name, items)| {
        json!({
            "name": name,
            "value": items.iter().take(5).cloned().collect::<Vec<_>>().join("\n"),
            "inline": false
        })
    })
    .collect()
}

async fn mark_digest_delivered(db: &PgPool, delivery_id: Uuid) -> Result<(), ApiError> {
    update_digest_status(db, delivery_id, "delivered", None).await
}

async fn mark_digest_skipped(db: &PgPool, delivery_id: Uuid) -> Result<(), ApiError> {
    update_digest_status(db, delivery_id, "skipped_empty", None).await
}

async fn mark_digest_failed(db: &PgPool, delivery_id: Uuid, error: &str) -> Result<(), ApiError> {
    update_digest_status(db, delivery_id, "failed", Some(error)).await
}

async fn update_digest_status(
    db: &PgPool,
    delivery_id: Uuid,
    status: &str,
    error: Option<&str>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        UPDATE notification_digest_deliveries
        SET status = $2, error = $3, delivered_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(delivery_id)
    .bind(status)
    .bind(error.map(|value| value.chars().take(240).collect::<String>()))
    .execute(db)
    .await?;
    Ok(())
}

async fn update_channel_error(
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

fn repo_names(rows: Vec<RadarCandidateRow>) -> Vec<String> {
    rows.into_iter()
        .map(|row| repo_name(row.owner, row.name))
        .collect()
}

fn repo_name(owner: Option<String>, name: Option<String>) -> String {
    match (owner, name) {
        (Some(owner), Some(name)) => format!("{owner}/{name}"),
        (_, Some(name)) => name,
        _ => "unknown repo".to_string(),
    }
}

fn push_unique(items: &mut Vec<String>, value: String) {
    if !items.contains(&value) {
        items.push(value);
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn digest_preset_maps_to_local_time() {
        assert_eq!(digest_time_for_preset("morning").unwrap(), "08:00");
        assert_eq!(digest_time_for_preset("noon").unwrap(), "12:00");
        assert_eq!(digest_time_for_preset("evening").unwrap(), "18:00");
        assert_eq!(digest_time_for_preset("night").unwrap(), "21:00");
        assert!(digest_time_for_preset("custom").is_err());
    }

    #[test]
    fn digest_window_matches_user_timezone() {
        let now = Utc.with_ymd_and_hms(2026, 5, 8, 6, 5, 0).unwrap();

        assert!(is_digest_due_now("08:00", "Europe/Paris", now, 30).unwrap());
        assert!(!is_digest_due_now("12:00", "Europe/Paris", now, 30).unwrap());
    }
}
