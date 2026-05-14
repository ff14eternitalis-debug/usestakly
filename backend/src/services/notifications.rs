use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::watchlist::{Notification, NotificationKind},
    services::notification_channels,
};

const SCORE_DROP_THRESHOLD: f64 = 0.10;
const ABANDONMENT_UP_THRESHOLD: f64 = 0.20;
const SEVERE_FLAGS: &[&str] = &["security-issue", "broken"];

#[derive(Debug, Clone)]
pub struct ScoreSnapshot {
    pub overall: f64,
    pub abandonment: f64,
    pub flags: Vec<String>,
}

pub async fn fetch_prev_snapshot(
    db: &PgPool,
    external_artifact_id: Uuid,
    formula_version: &str,
) -> Result<Option<ScoreSnapshot>, sqlx::Error> {
    let row: Option<(f64, f64, Option<Vec<String>>)> = sqlx::query_as(
        r#"
        SELECT overall::float8, abandonment::float8, flags
        FROM artifact_scores
        WHERE external_artifact_id = $1 AND formula_version = $2
        "#,
    )
    .bind(external_artifact_id)
    .bind(formula_version)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|(overall, abandonment, flags)| ScoreSnapshot {
        overall,
        abandonment,
        flags: flags.unwrap_or_default(),
    }))
}

pub async fn detect_and_emit(
    db: &PgPool,
    external_artifact_id: Uuid,
    prev: Option<&ScoreSnapshot>,
    new: &ScoreSnapshot,
    config: Option<&AppConfig>,
    notification_secret: Option<&str>,
) -> Result<usize, sqlx::Error> {
    let Some(prev) = prev else {
        return Ok(0);
    };

    let watchers: Vec<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT user_id
        FROM watched_artifacts
        WHERE external_artifact_id = $1 AND muted = FALSE
        "#,
    )
    .bind(external_artifact_id)
    .fetch_all(db)
    .await?;

    if watchers.is_empty() {
        return Ok(0);
    }

    let mut events: Vec<(NotificationKind, serde_json::Value)> = Vec::new();

    if new.overall <= prev.overall - SCORE_DROP_THRESHOLD {
        events.push((
            NotificationKind::ScoreDrop,
            json!({
                "prev_overall": prev.overall,
                "new_overall": new.overall,
                "delta": new.overall - prev.overall,
            }),
        ));
    }

    if new.abandonment >= prev.abandonment + ABANDONMENT_UP_THRESHOLD {
        events.push((
            NotificationKind::AbandonmentUp,
            json!({
                "prev_abandonment": prev.abandonment,
                "new_abandonment": new.abandonment,
                "delta": new.abandonment - prev.abandonment,
            }),
        ));
    }

    for flag in &new.flags {
        if prev.flags.contains(flag) {
            continue;
        }
        let kind = if SEVERE_FLAGS.contains(&flag.as_str()) {
            NotificationKind::FlagSevere
        } else {
            NotificationKind::FlagAdded
        };
        events.push((kind, json!({ "flag": flag })));
    }

    if events.is_empty() {
        return Ok(0);
    }

    let mut inserted = 0usize;
    for (user_id,) in &watchers {
        for (kind, payload) in &events {
            sqlx::query(
                r#"
                INSERT INTO notifications (user_id, external_artifact_id, kind, payload)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(user_id)
            .bind(external_artifact_id)
            .bind(kind)
            .bind(payload)
            .execute(db)
            .await?;
            inserted += 1;

            if let Some(secret) = notification_secret {
                deliver_external_alert(
                    db,
                    *user_id,
                    external_artifact_id,
                    *kind,
                    payload,
                    config,
                    secret,
                )
                .await;
            }
        }
    }

    Ok(inserted)
}

#[derive(FromRow)]
struct ExternalAlertRow {
    github_owner: Option<String>,
    github_repo: Option<String>,
    html_url: Option<String>,
}

async fn deliver_external_alert(
    db: &PgPool,
    user_id: Uuid,
    external_artifact_id: Uuid,
    kind: NotificationKind,
    payload: &serde_json::Value,
    config: Option<&AppConfig>,
    secret: &str,
) {
    let row: Result<Option<ExternalAlertRow>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT github_owner, github_repo, html_url
        FROM external_artifacts
        WHERE id = $1
        "#,
    )
    .bind(external_artifact_id)
    .fetch_optional(db)
    .await;

    let Some(row) = (match row {
        Ok(row) => row,
        Err(err) => {
            tracing::warn!(
                user_id = %user_id,
                artifact_id = %external_artifact_id,
                error = ?err,
                "failed to load repo for notification channel delivery"
            );
            return;
        }
    }) else {
        return;
    };

    let repo_full_name = match (row.github_owner, row.github_repo) {
        (Some(owner), Some(repo)) => format!("{owner}/{repo}"),
        (_, Some(repo)) => repo,
        _ => external_artifact_id.to_string(),
    };

    let Some(config) = config else {
        tracing::warn!(
            user_id = %user_id,
            artifact_id = %external_artifact_id,
            "skipping external notification delivery because AppConfig is missing"
        );
        return;
    };

    if let Err(err) = notification_channels::deliver_watch_alert(
        db,
        user_id,
        notification_channels::WatchAlertDelivery {
            secret,
            config,
            repo_full_name: &repo_full_name,
            repo_url: row.html_url.as_deref(),
            kind,
            payload,
        },
    )
    .await
    {
        tracing::warn!(
            user_id = %user_id,
            artifact_id = %external_artifact_id,
            error = ?err,
            "failed to deliver external notification channel alert"
        );
    }
}

#[derive(FromRow)]
struct NotificationRow {
    id: Uuid,
    artifact_id: Uuid,
    owner: Option<String>,
    name: Option<String>,
    kind: NotificationKind,
    payload: serde_json::Value,
    created_at: DateTime<Utc>,
    read_at: Option<DateTime<Utc>>,
}

impl From<NotificationRow> for Notification {
    fn from(row: NotificationRow) -> Self {
        Self {
            id: row.id,
            artifact_id: row.artifact_id,
            owner: row.owner,
            name: row.name,
            kind: row.kind,
            payload: row.payload,
            created_at: row.created_at,
            read_at: row.read_at,
        }
    }
}

pub async fn list_for_user(
    db: &PgPool,
    user_id: Uuid,
    unread_only: bool,
    limit: i64,
) -> Result<Vec<Notification>, ApiError> {
    let rows: Vec<NotificationRow> = sqlx::query_as(
        r#"
        SELECT
          n.id                      AS id,
          n.external_artifact_id    AS artifact_id,
          e.github_owner            AS owner,
          e.github_repo             AS name,
          n.kind                    AS kind,
          n.payload                 AS payload,
          n.created_at              AS created_at,
          n.read_at                 AS read_at
        FROM notifications n
        JOIN external_artifacts e ON e.id = n.external_artifact_id
        WHERE n.user_id = $1
          AND ($2 = FALSE OR n.read_at IS NULL)
        ORDER BY n.created_at DESC
        LIMIT $3
        "#,
    )
    .bind(user_id)
    .bind(unread_only)
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(Notification::from).collect())
}

pub async fn unread_count(db: &PgPool, user_id: Uuid) -> Result<i64, ApiError> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;
    Ok(count)
}

pub async fn mark_read(db: &PgPool, user_id: Uuid, notification_id: Uuid) -> Result<(), ApiError> {
    let rows = sqlx::query(
        "UPDATE notifications SET read_at = NOW() WHERE id = $1 AND user_id = $2 AND read_at IS NULL",
    )
    .bind(notification_id)
    .bind(user_id)
    .execute(db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError::not_found("Notification not found"));
    }
    Ok(())
}

pub async fn mark_all_read(db: &PgPool, user_id: Uuid) -> Result<u64, ApiError> {
    let rows = sqlx::query(
        "UPDATE notifications SET read_at = NOW() WHERE user_id = $1 AND read_at IS NULL",
    )
    .bind(user_id)
    .execute(db)
    .await?
    .rows_affected();
    Ok(rows)
}
