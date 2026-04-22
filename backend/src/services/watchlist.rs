use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{app::error::ApiError, domain::watchlist::WatchedRepo};

pub async fn add_watch(db: &PgPool, user_id: Uuid, artifact_id: Uuid) -> Result<Uuid, ApiError> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM external_artifacts WHERE id = $1 AND source = 'github')",
    )
    .bind(artifact_id)
    .fetch_one(db)
    .await?;
    if !exists {
        return Err(ApiError::not_found("Repo not found"));
    }

    let id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO watched_artifacts (user_id, external_artifact_id)
        VALUES ($1, $2)
        ON CONFLICT (user_id, external_artifact_id) DO UPDATE
          SET muted = FALSE
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(artifact_id)
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub async fn remove_watch(db: &PgPool, user_id: Uuid, artifact_id: Uuid) -> Result<(), ApiError> {
    let rows = sqlx::query(
        "DELETE FROM watched_artifacts WHERE user_id = $1 AND external_artifact_id = $2",
    )
    .bind(user_id)
    .bind(artifact_id)
    .execute(db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError::not_found("Not watched"));
    }
    Ok(())
}

pub async fn set_muted(
    db: &PgPool,
    user_id: Uuid,
    artifact_id: Uuid,
    muted: bool,
) -> Result<(), ApiError> {
    let rows = sqlx::query(
        "UPDATE watched_artifacts SET muted = $3 WHERE user_id = $1 AND external_artifact_id = $2",
    )
    .bind(user_id)
    .bind(artifact_id)
    .bind(muted)
    .execute(db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(ApiError::not_found("Not watched"));
    }
    Ok(())
}

pub async fn list_for_user(
    db: &PgPool,
    user_id: Uuid,
    formula_version: &str,
) -> Result<Vec<WatchedRepo>, ApiError> {
    let rows: Vec<WatchedRow> = sqlx::query_as(
        r#"
        SELECT
          w.id                        AS id,
          w.external_artifact_id      AS artifact_id,
          w.muted                     AS muted,
          w.created_at                AS watched_at,
          e.github_owner              AS owner,
          e.github_repo               AS name,
          e.html_url                  AS html_url,
          e.language                  AS language,
          e.stars_count               AS stars_count,
          e.archived                  AS archived,
          e.last_commit_at            AS last_commit_at,
          ascore.overall::float8      AS overall,
          ascore.abandonment::float8  AS abandonment,
          ascore.flags                AS flags
        FROM watched_artifacts w
        JOIN external_artifacts e ON e.id = w.external_artifact_id
        LEFT JOIN artifact_scores ascore
          ON ascore.external_artifact_id = e.id
          AND ascore.formula_version = $2
        WHERE w.user_id = $1
        ORDER BY w.created_at DESC
        "#,
    )
    .bind(user_id)
    .bind(formula_version)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(WatchedRow::into_repo).collect())
}

#[derive(FromRow)]
struct WatchedRow {
    id: Uuid,
    artifact_id: Uuid,
    muted: bool,
    watched_at: DateTime<Utc>,
    owner: Option<String>,
    name: Option<String>,
    html_url: Option<String>,
    language: Option<String>,
    stars_count: i32,
    archived: bool,
    last_commit_at: Option<DateTime<Utc>>,
    overall: Option<f64>,
    abandonment: Option<f64>,
    flags: Option<Vec<String>>,
}

impl WatchedRow {
    fn into_repo(self) -> WatchedRepo {
        let owner = self.owner.unwrap_or_default();
        let name = self.name.unwrap_or_default();
        let full_name = format!("{owner}/{name}");
        let html_url = self
            .html_url
            .unwrap_or_else(|| format!("https://github.com/{full_name}"));
        WatchedRepo {
            id: self.id,
            artifact_id: self.artifact_id,
            owner,
            name,
            full_name,
            html_url,
            language: self.language,
            stars_count: self.stars_count,
            archived: self.archived,
            last_commit_at: self.last_commit_at,
            muted: self.muted,
            watched_at: self.watched_at,
            overall: self.overall,
            abandonment: self.abandonment,
            flags: self.flags.unwrap_or_default(),
        }
    }
}
