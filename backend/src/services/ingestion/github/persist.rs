use octocrab::Octocrab;
use sqlx::PgPool;
use uuid::Uuid;

use super::repo::{GitHubRepoMetadata, fetch_repo_with_state};
use super::structural::{ExistingGitHubIngestionState, fetch_readme_text_with_etag};
use crate::{
    app::error::ApiError,
    config::AppConfig,
    services::{repo_categories, semantic_search},
};

pub async fn upsert_github_artifact(
    db: &PgPool,
    meta: &GitHubRepoMetadata,
) -> Result<Uuid, ApiError> {
    let canonical_slug = format!("github:{}/{}", meta.owner, meta.name);
    let package_name = format!("{}/{}", meta.owner, meta.name);

    let id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO external_artifacts (
          source, canonical_slug, package_name,
          github_id, github_owner, github_repo,
          default_branch, html_url, description, language,
          license_spdx, topics, archived,
          stars_count, forks_count, open_issues_count, subscribers_count,
          last_commit_at, priors_fetched_at,
          distinct_contributors_90d, commits_30d, has_ci,
          releases_count, last_release_at, structural_signals_at,
          github_releases_etag, github_readme_etag, github_events_etag,
          github_rate_limit_reset_at, github_last_rate_limit_at,
          owner_last_activity_at, owner_inactive_days
        )
        VALUES (
          CAST($1 AS external_source), $2, $3,
          $4, $5, $6,
          $7, $8, $9, $10,
          $11, $12, $13,
          $14, $15, $16, $17,
          $18, NOW(),
          $19, $20, $21,
          $22, $23, $24,
          $25, $26, $27,
          $28, $29,
          $30, $31
        )
        ON CONFLICT (source, canonical_slug) DO UPDATE SET
          package_name = EXCLUDED.package_name,
          github_id = EXCLUDED.github_id,
          github_owner = EXCLUDED.github_owner,
          github_repo = EXCLUDED.github_repo,
          default_branch = EXCLUDED.default_branch,
          html_url = EXCLUDED.html_url,
          description = EXCLUDED.description,
          language = EXCLUDED.language,
          license_spdx = EXCLUDED.license_spdx,
          topics = EXCLUDED.topics,
          archived = EXCLUDED.archived,
          stars_count = EXCLUDED.stars_count,
          forks_count = EXCLUDED.forks_count,
          open_issues_count = EXCLUDED.open_issues_count,
          subscribers_count = EXCLUDED.subscribers_count,
          last_commit_at = EXCLUDED.last_commit_at,
          priors_fetched_at = NOW(),
          distinct_contributors_90d = COALESCE(EXCLUDED.distinct_contributors_90d, external_artifacts.distinct_contributors_90d),
          commits_30d              = COALESCE(EXCLUDED.commits_30d,              external_artifacts.commits_30d),
          has_ci                   = COALESCE(EXCLUDED.has_ci,                   external_artifacts.has_ci),
          releases_count           = COALESCE(EXCLUDED.releases_count,           external_artifacts.releases_count),
          last_release_at          = COALESCE(EXCLUDED.last_release_at,          external_artifacts.last_release_at),
          structural_signals_at    = COALESCE(EXCLUDED.structural_signals_at,    external_artifacts.structural_signals_at),
          github_releases_etag     = COALESCE(EXCLUDED.github_releases_etag,     external_artifacts.github_releases_etag),
          github_readme_etag       = COALESCE(EXCLUDED.github_readme_etag,       external_artifacts.github_readme_etag),
          github_events_etag       = COALESCE(EXCLUDED.github_events_etag,       external_artifacts.github_events_etag),
          github_rate_limit_reset_at = COALESCE(EXCLUDED.github_rate_limit_reset_at, external_artifacts.github_rate_limit_reset_at),
          github_last_rate_limit_at  = COALESCE(EXCLUDED.github_last_rate_limit_at,  external_artifacts.github_last_rate_limit_at),
          owner_last_activity_at   = COALESCE(EXCLUDED.owner_last_activity_at,   external_artifacts.owner_last_activity_at),
          owner_inactive_days      = COALESCE(EXCLUDED.owner_inactive_days,      external_artifacts.owner_inactive_days)
        RETURNING id
        "#,
    )
    .bind("github")
    .bind(&canonical_slug)
    .bind(&package_name)
    .bind(meta.github_id)
    .bind(&meta.owner)
    .bind(&meta.name)
    .bind(&meta.default_branch)
    .bind(&meta.html_url)
    .bind(&meta.description)
    .bind(&meta.language)
    .bind(&meta.license_spdx)
    .bind(&meta.topics)
    .bind(meta.archived)
    .bind(meta.stars_count)
    .bind(meta.forks_count)
    .bind(meta.open_issues_count)
    .bind(meta.subscribers_count)
    .bind(meta.last_commit_at)
    .bind(meta.structural.distinct_contributors_90d)
    .bind(meta.structural.commits_30d)
    .bind(meta.structural.has_ci)
    .bind(meta.structural.releases_count)
    .bind(meta.structural.last_release_at)
    .bind(meta.structural.captured_at)
    .bind(&meta.ingestion.releases_etag)
    .bind(&meta.ingestion.readme_etag)
    .bind(&meta.ingestion.events_etag)
    .bind(meta.ingestion.rate_limit_reset_at)
    .bind(meta.ingestion.last_rate_limit_at)
    .bind(meta.structural.owner_last_activity_at)
    .bind(meta.structural.owner_inactive_days)
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub(crate) async fn load_existing_ingestion_state(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<ExistingGitHubIngestionState, ApiError> {
    let canonical_slug = format!("github:{owner}/{name}");
    let state = sqlx::query_as::<_, ExistingGitHubIngestionState>(
        r#"
        SELECT
          etag                    AS repo_etag,
          github_releases_etag    AS releases_etag,
          github_readme_etag      AS readme_etag,
          github_events_etag      AS events_etag,
          releases_count          AS releases_count,
          last_release_at         AS last_release_at,
          owner_last_activity_at  AS owner_last_activity_at,
          owner_inactive_days     AS owner_inactive_days
        FROM external_artifacts
        WHERE source = 'github'
          AND canonical_slug = $1
        "#,
    )
    .bind(canonical_slug)
    .fetch_optional(db)
    .await?;
    Ok(state.unwrap_or_default())
}

pub async fn ingest_repo(
    client: &Octocrab,
    db: &PgPool,
    config: &AppConfig,
    owner: &str,
    name: &str,
) -> Result<
    (
        Uuid,
        GitHubRepoMetadata,
        Vec<crate::domain::repo::RepoCategory>,
    ),
    ApiError,
> {
    let existing = load_existing_ingestion_state(db, owner, name).await?;
    let mut meta = fetch_repo_with_state(client, owner, name, &existing).await?;
    let readme = fetch_readme_text_with_etag(
        client,
        &meta.owner,
        &meta.name,
        existing.readme_etag.as_deref(),
    )
    .await
    .map(|readme| {
        meta.ingestion.readme_etag = readme.etag.or(existing.readme_etag.clone());
        readme.content
    })
    .unwrap_or_else(|err| {
        tracing::warn!(
            target: "ingestion::github::readme",
            "README fetch failed for {}/{}: {err:?}",
            meta.owner,
            meta.name
        );
        None
    });
    let id = upsert_github_artifact(db, &meta).await?;
    let categories =
        repo_categories::upsert_repo_categories_with_readme(db, id, &meta, readme.as_deref())
            .await?;
    if let Some(embedding) = semantic_search::embed_passage(
        semantic_search::build_search_document(
            &meta.owner,
            &meta.name,
            meta.description.as_deref(),
            meta.language.as_deref(),
            &meta.topics,
        ),
        config,
    )
    .await?
    {
        semantic_search::update_repo_embedding(db, id, &embedding).await?;
    }
    Ok((id, meta, categories))
}
