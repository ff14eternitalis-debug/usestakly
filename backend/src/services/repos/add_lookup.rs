use chrono::{DateTime, Utc};
use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use crate::{app::error::ApiError, domain::repo::RepoCategory, services::quality::load_v2};

#[derive(sqlx::FromRow)]
pub struct IndexedRepoAddRow {
    pub artifact_id: Uuid,
    pub owner: String,
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub default_branch: Option<String>,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub categories: Json<Vec<RepoCategory>>,
}

pub async fn find_github_artifact_id(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<Option<Uuid>, ApiError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id
        FROM external_artifacts
        WHERE source = 'github'
          AND LOWER(github_owner) = LOWER($1)
          AND LOWER(github_repo) = LOWER($2)
        LIMIT 1
        "#,
    )
    .bind(owner)
    .bind(name)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|(id,)| id))
}

pub async fn load_indexed_repo_for_add(
    db: &PgPool,
    artifact_id: Uuid,
) -> Result<IndexedRepoAddRow, ApiError> {
    sqlx::query_as::<_, IndexedRepoAddRow>(
        r#"
        SELECT
          e.id AS artifact_id,
          e.github_owner AS owner,
          e.github_repo AS name,
          e.html_url,
          e.description,
          e.language,
          e.license_spdx,
          e.topics,
          e.stars_count,
          e.forks_count,
          e.open_issues_count,
          e.subscribers_count,
          e.archived,
          e.default_branch,
          e.last_commit_at,
          COALESCE((
            SELECT jsonb_agg(
              jsonb_build_object(
                'category', rc.category,
                'confidence', rc.confidence,
                'source', rc.source,
                'evidence', rc.evidence
              )
              ORDER BY rc.confidence DESC, rc.category
            )
            FROM repo_categories rc
            WHERE rc.external_artifact_id = e.id
          ), '[]'::jsonb) AS categories
        FROM external_artifacts e
        WHERE e.id = $1 AND e.source = 'github'
        "#,
    )
    .bind(artifact_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::not_found("Repo not found"))
}

/// When true, `POST /api/repos/add` must not call `ingest_repo` (no GitHub quota spent).
pub fn should_short_circuit_github_ingest(already_indexed: bool) -> bool {
    already_indexed
}

pub fn formula_version_for_add() -> Result<String, ApiError> {
    Ok(load_v2()
        .map_err(|e| ApiError::internal(format!("loading formula: {e}")))?
        .meta
        .version)
}

#[cfg(test)]
mod tests {
    use super::should_short_circuit_github_ingest;

    #[test]
    fn short_circuit_skips_github_when_repo_already_indexed() {
        assert!(should_short_circuit_github_ingest(true));
        assert!(!should_short_circuit_github_ingest(false));
    }
}
