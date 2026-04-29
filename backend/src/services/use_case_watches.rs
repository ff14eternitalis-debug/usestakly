use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    services::recommendations::{UseCaseIntent, recommend_for_use_case},
};

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
              use_case_watch_id, external_artifact_id, match_score, quality_score
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (use_case_watch_id, external_artifact_id) DO UPDATE
              SET match_score = EXCLUDED.match_score,
                  quality_score = EXCLUDED.quality_score,
                  last_seen_at = NOW()
            "#,
        )
        .bind(watch_id)
        .bind(recommendation.repo.artifact_id)
        .bind(recommendation.match_score)
        .bind(recommendation.repo.quality.as_ref().and_then(|q| q.overall))
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
}
