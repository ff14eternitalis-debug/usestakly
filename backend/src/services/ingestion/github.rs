use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::error::ApiError;

pub struct GitHubRepoMetadata {
    pub github_id: i64,
    pub owner: String,
    pub name: String,
    pub default_branch: Option<String>,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub archived: bool,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub last_commit_at: Option<DateTime<Utc>>,
}

pub fn build_client(token: &str) -> Result<Octocrab, ApiError> {
    Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .map_err(|e| ApiError::internal(format!("github client build failed: {e}")))
}

pub async fn fetch_repo(
    client: &Octocrab,
    owner: &str,
    name: &str,
) -> Result<GitHubRepoMetadata, ApiError> {
    let repo = client.repos(owner, name).get().await.map_err(|e| match e {
        octocrab::Error::GitHub { source, .. } if source.status_code.as_u16() == 404 => {
            ApiError::not_found(format!("github repo not found: {owner}/{name}"))
        }
        other => ApiError::internal(format!("github fetch failed: {other}")),
    })?;

    let language = repo
        .language
        .as_ref()
        .and_then(|v| v.as_str().map(String::from));

    let license_spdx = repo
        .license
        .as_ref()
        .map(|l| l.spdx_id.clone())
        .filter(|s: &String| !s.is_empty() && s != "NOASSERTION");

    let resolved_owner = repo
        .owner
        .as_ref()
        .map(|o| o.login.clone())
        .unwrap_or_else(|| owner.to_string());

    let html_url = repo
        .html_url
        .as_ref()
        .map(|u| u.to_string())
        .unwrap_or_else(|| format!("https://github.com/{resolved_owner}/{}", repo.name));

    Ok(GitHubRepoMetadata {
        github_id: *repo.id as i64,
        owner: resolved_owner,
        name: repo.name,
        default_branch: repo.default_branch,
        html_url,
        description: repo.description,
        language,
        license_spdx,
        topics: repo.topics.unwrap_or_default(),
        archived: repo.archived.unwrap_or(false),
        stars_count: repo.stargazers_count.unwrap_or(0) as i32,
        forks_count: repo.forks_count.unwrap_or(0) as i32,
        open_issues_count: repo.open_issues_count.unwrap_or(0) as i32,
        subscribers_count: repo.subscribers_count.unwrap_or(0) as i32,
        last_commit_at: repo.pushed_at,
    })
}

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
          last_commit_at, priors_fetched_at
        )
        VALUES (
          CAST($1 AS external_source), $2, $3,
          $4, $5, $6,
          $7, $8, $9, $10,
          $11, $12, $13,
          $14, $15, $16, $17,
          $18, NOW()
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
          priors_fetched_at = NOW()
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
    .fetch_one(db)
    .await?;

    Ok(id)
}

pub async fn ingest_repo(
    client: &Octocrab,
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<(Uuid, GitHubRepoMetadata), ApiError> {
    let meta = fetch_repo(client, owner, name).await?;
    let id = upsert_github_artifact(db, &meta).await?;
    Ok((id, meta))
}
