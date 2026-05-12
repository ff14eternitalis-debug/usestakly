use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::watchlist::NotificationKind,
    services::recommendations::{UseCaseIntent, recommend_for_use_case},
};

const USE_CASE_QUALITY_DROP_THRESHOLD: f64 = 0.10;
const USE_CASE_NOTIFICATION_COOLDOWN_HOURS: i64 = 24;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseWatch {
    pub id: Uuid,
    pub query_text: String,
    pub label: String,
    pub normalized_intent: String,
    pub categories: Vec<String>,
    pub topics: Vec<String>,
    pub languages: Vec<String>,
    pub risk_tolerance: String,
    pub enabled: bool,
    pub match_count: i64,
    pub top_matches: Vec<UseCaseWatchMatch>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseWatchMatch {
    pub artifact_id: Uuid,
    pub full_name: String,
    pub language: Option<String>,
    pub match_score: f64,
    pub quality_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseCaseWatchEvent {
    NewCandidate {
        artifact_id: Uuid,
    },
    BestCandidateChanged {
        previous_artifact_id: Uuid,
        current_artifact_id: Uuid,
    },
    QualityDrop {
        artifact_id: Uuid,
        previous_quality: f64,
        current_quality: f64,
    },
    FlagAdded {
        artifact_id: Uuid,
        flag: String,
    },
}

#[derive(Debug, Clone, FromRow)]
pub struct StoredUseCaseMatch {
    pub artifact_id: Uuid,
    pub match_score: f64,
    pub quality_score: Option<f64>,
    pub flags: Vec<String>,
}

pub fn detect_use_case_watch_events(
    previous: &[StoredUseCaseMatch],
    current: &[StoredUseCaseMatch],
    last_notified_hours_ago: Option<i64>,
) -> Vec<UseCaseWatchEvent> {
    if last_notified_hours_ago.is_some_and(|hours| hours < USE_CASE_NOTIFICATION_COOLDOWN_HOURS) {
        return Vec::new();
    }

    let mut events = Vec::new();
    if let (Some(previous_best), Some(current_best)) = (previous.first(), current.first())
        && previous_best.artifact_id != current_best.artifact_id
    {
        events.push(UseCaseWatchEvent::BestCandidateChanged {
            previous_artifact_id: previous_best.artifact_id,
            current_artifact_id: current_best.artifact_id,
        });
    }

    for current_match in current {
        let previous_match = previous
            .iter()
            .find(|item| item.artifact_id == current_match.artifact_id);
        let Some(previous_match) = previous_match else {
            events.push(UseCaseWatchEvent::NewCandidate {
                artifact_id: current_match.artifact_id,
            });
            continue;
        };

        if let (Some(previous_quality), Some(current_quality)) =
            (previous_match.quality_score, current_match.quality_score)
            && current_quality <= previous_quality - USE_CASE_QUALITY_DROP_THRESHOLD
        {
            events.push(UseCaseWatchEvent::QualityDrop {
                artifact_id: current_match.artifact_id,
                previous_quality,
                current_quality,
            });
        }

        for flag in &current_match.flags {
            if !previous_match.flags.contains(flag) {
                events.push(UseCaseWatchEvent::FlagAdded {
                    artifact_id: current_match.artifact_id,
                    flag: flag.clone(),
                });
            }
        }
    }

    events
}

pub fn default_watch_label(intent: &UseCaseIntent) -> String {
    if intent.label == "Recherche OSS" {
        return "Veille OSS".to_string();
    }
    format!("Veille {}", intent.label)
}

pub async fn create_watch(
    db: &PgPool,
    config: &AppConfig,
    user_id: Uuid,
    query: &str,
    label: Option<String>,
    risk_tolerance: &str,
) -> Result<UseCaseWatch, ApiError> {
    let report = recommend_for_use_case(db, config, query, risk_tolerance, 8).await?;
    let label = label
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default_watch_label(&report.intent));

    let mut tx = db.begin().await?;
    let query_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO use_case_queries (
          user_id, query_text, normalized_intent, categories, topics, languages, risk_tolerance
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(&report.query)
    .bind(&report.intent.label)
    .bind(&report.intent.categories)
    .bind(&report.intent.topics)
    .bind(&report.intent.languages)
    .bind(&report.risk_tolerance)
    .fetch_one(&mut *tx)
    .await?;

    let watch_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO use_case_watches (user_id, use_case_query_id, label, last_checked_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(query_id)
    .bind(&label)
    .fetch_one(&mut *tx)
    .await?;

    for recommendation in &report.recommendations {
        sqlx::query(
            r#"
            INSERT INTO use_case_watch_matches (
              use_case_watch_id, external_artifact_id, match_score, quality_score, flags
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (use_case_watch_id, external_artifact_id) DO UPDATE
              SET match_score = EXCLUDED.match_score,
                  quality_score = EXCLUDED.quality_score,
                  flags = EXCLUDED.flags,
                  last_seen_at = NOW()
            "#,
        )
        .bind(watch_id)
        .bind(recommendation.repo.artifact_id)
        .bind(recommendation.match_score)
        .bind(recommendation.repo.quality.as_ref().and_then(|q| q.overall))
        .bind(
            recommendation
                .repo
                .quality
                .as_ref()
                .map(|q| q.flags.clone())
                .unwrap_or_default(),
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    get_watch(db, user_id, watch_id).await
}

pub async fn list_watches(db: &PgPool, user_id: Uuid) -> Result<Vec<UseCaseWatch>, ApiError> {
    let rows: Vec<UseCaseWatchRow> = sqlx::query_as(
        r#"
        SELECT
          w.id,
          q.query_text,
          w.label,
          q.normalized_intent,
          q.categories,
          q.topics,
          q.languages,
          q.risk_tolerance,
          w.enabled,
          w.created_at,
          (
            SELECT COUNT(*)::bigint
            FROM use_case_watch_matches m
            WHERE m.use_case_watch_id = w.id
          ) AS match_count
        FROM use_case_watches w
        JOIN use_case_queries q ON q.id = w.use_case_query_id
        WHERE w.user_id = $1
        ORDER BY w.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    let mut watches = Vec::with_capacity(rows.len());
    for row in rows {
        let watch_id = row.id;
        watches.push(row.into_watch(list_matches(db, watch_id).await?));
    }
    Ok(watches)
}

#[derive(Debug, FromRow)]
struct EnabledUseCaseWatchRow {
    id: Uuid,
    user_id: Uuid,
    query_text: String,
    risk_tolerance: String,
    last_notified_at: Option<DateTime<Utc>>,
}

pub async fn evaluate_enabled_watches(db: &PgPool, config: &AppConfig) -> Result<usize, ApiError> {
    let rows: Vec<EnabledUseCaseWatchRow> = sqlx::query_as(
        r#"
        SELECT w.id, w.user_id, q.query_text, q.risk_tolerance, w.last_notified_at
        FROM use_case_watches w
        JOIN use_case_queries q ON q.id = w.use_case_query_id
        WHERE w.enabled = TRUE
        ORDER BY w.last_checked_at ASC NULLS FIRST, w.created_at ASC
        "#,
    )
    .fetch_all(db)
    .await?;

    let mut inserted = 0usize;
    for row in rows {
        let previous = stored_matches(db, row.id).await?;
        let report =
            recommend_for_use_case(db, config, &row.query_text, &row.risk_tolerance, 8).await?;
        let current: Vec<StoredUseCaseMatch> = report
            .recommendations
            .iter()
            .map(|recommendation| StoredUseCaseMatch {
                artifact_id: recommendation.repo.artifact_id,
                match_score: recommendation.match_score,
                quality_score: recommendation.repo.quality.as_ref().and_then(|q| q.overall),
                flags: recommendation
                    .repo
                    .quality
                    .as_ref()
                    .map(|q| q.flags.clone())
                    .unwrap_or_default(),
            })
            .collect();
        let events = detect_use_case_watch_events(
            &previous,
            &current,
            last_notified_hours_ago(row.last_notified_at, Utc::now()),
        );
        inserted += persist_watch_evaluation(db, &row, &current, &events).await?;
    }

    Ok(inserted)
}

async fn stored_matches(db: &PgPool, watch_id: Uuid) -> Result<Vec<StoredUseCaseMatch>, ApiError> {
    let rows = sqlx::query_as(
        r#"
        SELECT
          external_artifact_id AS artifact_id,
          match_score::float8 AS match_score,
          quality_score::float8 AS quality_score,
          flags
        FROM use_case_watch_matches
        WHERE use_case_watch_id = $1
        ORDER BY quality_score DESC NULLS LAST, match_score DESC, last_seen_at DESC
        "#,
    )
    .bind(watch_id)
    .fetch_all(db)
    .await?;
    Ok(rows)
}

async fn persist_watch_evaluation(
    db: &PgPool,
    watch: &EnabledUseCaseWatchRow,
    current: &[StoredUseCaseMatch],
    events: &[UseCaseWatchEvent],
) -> Result<usize, ApiError> {
    let mut tx = db.begin().await?;
    let current_ids: Vec<Uuid> = current.iter().map(|item| item.artifact_id).collect();

    if current_ids.is_empty() {
        sqlx::query("DELETE FROM use_case_watch_matches WHERE use_case_watch_id = $1")
            .bind(watch.id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query(
            r#"
            DELETE FROM use_case_watch_matches
            WHERE use_case_watch_id = $1
              AND NOT (external_artifact_id = ANY($2))
            "#,
        )
        .bind(watch.id)
        .bind(&current_ids)
        .execute(&mut *tx)
        .await?;
    }

    for item in current {
        sqlx::query(
            r#"
            INSERT INTO use_case_watch_matches (
              use_case_watch_id, external_artifact_id, match_score, quality_score, flags
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (use_case_watch_id, external_artifact_id) DO UPDATE
              SET match_score = EXCLUDED.match_score,
                  quality_score = EXCLUDED.quality_score,
                  flags = EXCLUDED.flags,
                  last_seen_at = NOW()
            "#,
        )
        .bind(watch.id)
        .bind(item.artifact_id)
        .bind(item.match_score)
        .bind(item.quality_score)
        .bind(&item.flags)
        .execute(&mut *tx)
        .await?;
    }

    let mut inserted = 0usize;
    for event in events {
        let (artifact_id, kind, payload) = event_notification(event);
        sqlx::query(
            r#"
            INSERT INTO notifications (user_id, external_artifact_id, kind, payload)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(watch.user_id)
        .bind(artifact_id)
        .bind(kind)
        .bind(payload)
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }

    if inserted > 0 {
        sqlx::query(
            "UPDATE use_case_watches SET last_checked_at = NOW(), last_notified_at = NOW() WHERE id = $1",
        )
        .bind(watch.id)
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query("UPDATE use_case_watches SET last_checked_at = NOW() WHERE id = $1")
            .bind(watch.id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(inserted)
}

fn last_notified_hours_ago(
    last_notified_at: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
) -> Option<i64> {
    last_notified_at.map(|last| now.signed_duration_since(last).num_hours().max(0))
}

fn event_notification(event: &UseCaseWatchEvent) -> (Uuid, NotificationKind, serde_json::Value) {
    match event {
        UseCaseWatchEvent::NewCandidate { artifact_id } => (
            *artifact_id,
            NotificationKind::UseCaseNewCandidate,
            serde_json::json!({ "artifact_id": artifact_id }),
        ),
        UseCaseWatchEvent::BestCandidateChanged {
            previous_artifact_id,
            current_artifact_id,
        } => (
            *current_artifact_id,
            NotificationKind::UseCaseBestCandidateChanged,
            serde_json::json!({
                "previous_artifact_id": previous_artifact_id,
                "current_artifact_id": current_artifact_id,
            }),
        ),
        UseCaseWatchEvent::QualityDrop {
            artifact_id,
            previous_quality,
            current_quality,
        } => (
            *artifact_id,
            NotificationKind::UseCaseQualityDrop,
            serde_json::json!({
                "previous_quality": previous_quality,
                "current_quality": current_quality,
                "delta": current_quality - previous_quality,
            }),
        ),
        UseCaseWatchEvent::FlagAdded { artifact_id, flag } => (
            *artifact_id,
            NotificationKind::UseCaseFlagAdded,
            serde_json::json!({ "flag": flag }),
        ),
    }
}

async fn get_watch(db: &PgPool, user_id: Uuid, watch_id: Uuid) -> Result<UseCaseWatch, ApiError> {
    let row: UseCaseWatchRow = sqlx::query_as(
        r#"
        SELECT
          w.id,
          q.query_text,
          w.label,
          q.normalized_intent,
          q.categories,
          q.topics,
          q.languages,
          q.risk_tolerance,
          w.enabled,
          w.created_at,
          (
            SELECT COUNT(*)::bigint
            FROM use_case_watch_matches m
            WHERE m.use_case_watch_id = w.id
          ) AS match_count
        FROM use_case_watches w
        JOIN use_case_queries q ON q.id = w.use_case_query_id
        WHERE w.user_id = $1 AND w.id = $2
        "#,
    )
    .bind(user_id)
    .bind(watch_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Use-case watch not found"))?;

    let watch_id = row.id;
    Ok(row.into_watch(list_matches(db, watch_id).await?))
}

async fn list_matches(db: &PgPool, watch_id: Uuid) -> Result<Vec<UseCaseWatchMatch>, ApiError> {
    let rows: Vec<UseCaseWatchMatchRow> = sqlx::query_as(
        r#"
        SELECT
          m.external_artifact_id AS artifact_id,
          COALESCE(e.github_owner, '') AS owner,
          COALESCE(e.github_repo, '') AS name,
          e.language,
          m.match_score::float8 AS match_score,
          m.quality_score::float8 AS quality_score
        FROM use_case_watch_matches m
        JOIN external_artifacts e ON e.id = m.external_artifact_id
        WHERE m.use_case_watch_id = $1
        ORDER BY m.quality_score DESC NULLS LAST, m.match_score DESC
        LIMIT 5
        "#,
    )
    .bind(watch_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(UseCaseWatchMatchRow::into_match)
        .collect())
}

#[derive(Debug, FromRow)]
struct UseCaseWatchRow {
    id: Uuid,
    query_text: String,
    label: String,
    normalized_intent: String,
    categories: Vec<String>,
    topics: Vec<String>,
    languages: Vec<String>,
    risk_tolerance: String,
    enabled: bool,
    created_at: DateTime<Utc>,
    match_count: i64,
}

impl UseCaseWatchRow {
    fn into_watch(self, top_matches: Vec<UseCaseWatchMatch>) -> UseCaseWatch {
        UseCaseWatch {
            id: self.id,
            query_text: self.query_text,
            label: self.label,
            normalized_intent: self.normalized_intent,
            categories: self.categories,
            topics: self.topics,
            languages: self.languages,
            risk_tolerance: self.risk_tolerance,
            enabled: self.enabled,
            match_count: self.match_count,
            top_matches,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct UseCaseWatchMatchRow {
    artifact_id: Uuid,
    owner: String,
    name: String,
    language: Option<String>,
    match_score: f64,
    quality_score: Option<f64>,
}

impl UseCaseWatchMatchRow {
    fn into_match(self) -> UseCaseWatchMatch {
        UseCaseWatchMatch {
            artifact_id: self.artifact_id,
            full_name: format!("{}/{}", self.owner, self.name),
            language: self.language,
            match_score: self.match_score,
            quality_score: self.quality_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::recommendations::{IntentConfidence, UseCaseIntent};

    use super::*;

    impl StoredUseCaseMatch {
        fn new_for_test(
            artifact_id: Uuid,
            match_score: f64,
            quality_score: Option<f64>,
            flags: Vec<String>,
        ) -> Self {
            Self {
                artifact_id,
                match_score,
                quality_score,
                flags,
            }
        }
    }

    #[test]
    fn default_label_uses_detected_intent() {
        let intent = UseCaseIntent {
            label: "ORM TypeScript".to_string(),
            confidence: IntentConfidence::High,
            categories: vec![],
            topics: vec![],
            languages: vec![],
        };

        assert_eq!(default_watch_label(&intent), "Veille ORM TypeScript");
    }

    #[test]
    fn watch_event_detection_reports_new_candidate_and_best_change() {
        let previous = vec![
            StoredUseCaseMatch::new_for_test(Uuid::from_u128(1), 0.91, Some(0.82), vec![]),
            StoredUseCaseMatch::new_for_test(Uuid::from_u128(2), 0.80, Some(0.76), vec![]),
        ];
        let current = vec![
            StoredUseCaseMatch::new_for_test(Uuid::from_u128(3), 0.94, Some(0.88), vec![]),
            StoredUseCaseMatch::new_for_test(Uuid::from_u128(1), 0.91, Some(0.82), vec![]),
        ];

        let events = detect_use_case_watch_events(&previous, &current, None);

        assert_eq!(
            events,
            vec![
                UseCaseWatchEvent::BestCandidateChanged {
                    previous_artifact_id: Uuid::from_u128(1),
                    current_artifact_id: Uuid::from_u128(3),
                },
                UseCaseWatchEvent::NewCandidate {
                    artifact_id: Uuid::from_u128(3),
                },
            ]
        );
    }

    #[test]
    fn watch_event_detection_reports_quality_drop_and_new_flag() {
        let repo_id = Uuid::from_u128(42);
        let previous = vec![StoredUseCaseMatch::new_for_test(
            repo_id,
            0.90,
            Some(0.86),
            vec!["deprecated".to_string()],
        )];
        let current = vec![StoredUseCaseMatch::new_for_test(
            repo_id,
            0.90,
            Some(0.73),
            vec!["deprecated".to_string(), "broken".to_string()],
        )];

        let events = detect_use_case_watch_events(&previous, &current, None);

        assert_eq!(
            events,
            vec![
                UseCaseWatchEvent::QualityDrop {
                    artifact_id: repo_id,
                    previous_quality: 0.86,
                    current_quality: 0.73,
                },
                UseCaseWatchEvent::FlagAdded {
                    artifact_id: repo_id,
                    flag: "broken".to_string(),
                },
            ]
        );
    }

    #[test]
    fn watch_event_detection_suppresses_events_after_recent_notification() {
        let previous = vec![StoredUseCaseMatch::new_for_test(
            Uuid::from_u128(1),
            0.90,
            Some(0.90),
            vec![],
        )];
        let current = vec![StoredUseCaseMatch::new_for_test(
            Uuid::from_u128(2),
            0.95,
            Some(0.92),
            vec![],
        )];

        let events = detect_use_case_watch_events(&previous, &current, Some(23));

        assert!(events.is_empty());
    }
}
