use std::collections::HashSet;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use chrono::{DateTime, Duration, Utc};
use octocrab::Octocrab;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    services::{repo_categories, semantic_search},
};

const COMMITS_LOOKBACK_DAYS: i64 = 90;
const COMMITS_30D_WINDOW: i64 = 30;
const COMMITS_PER_PAGE: u8 = 100;
const COMMITS_MAX_PAGES: u32 = 5;
const README_CLASSIFICATION_MAX_BYTES: usize = 80_000;

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
    pub structural: StructuralSignals,
}

/// Passive structural signals captured at ingestion time.
/// Each field is `Option` : a fetch failure (rate-limit, transient network) leaves
/// the slot at NULL rather than breaking the whole ingestion. The scoring formula
/// treats NULL as "unknown / neutral", not as "zero".
#[derive(Debug, Clone, Default)]
pub struct StructuralSignals {
    pub distinct_contributors_90d: Option<i32>,
    pub commits_30d: Option<i32>,
    pub has_ci: Option<bool>,
    pub releases_count: Option<i32>,
    pub last_release_at: Option<DateTime<Utc>>,
    pub captured_at: Option<DateTime<Utc>>,
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
        octocrab::Error::GitHub { source, .. }
            if source.status_code.as_u16() == 403 || source.status_code.as_u16() == 429 =>
        {
            ApiError::forbidden(
                "GitHub API rate limit reached or access denied; retry later or verify GITHUB_TOKEN",
            )
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

    let structural = fetch_structural_signals(client, &resolved_owner, &repo.name).await;

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
        structural,
    })
}

#[derive(Debug, Clone)]
struct CommitSummary {
    author_key: String,
    committed_at: DateTime<Utc>,
}

struct CommitTally {
    commits_30d: i32,
    distinct_contributors_90d: i32,
}

#[derive(Debug, Deserialize)]
struct GitHubReadmeResponse {
    content: Option<String>,
    encoding: Option<String>,
}

fn tally_commits(commits: &[CommitSummary], cutoff_30d: DateTime<Utc>) -> CommitTally {
    let mut authors: HashSet<&str> = HashSet::new();
    let mut commits_30d: i32 = 0;
    for commit in commits {
        authors.insert(commit.author_key.as_str());
        if commit.committed_at >= cutoff_30d {
            commits_30d = commits_30d.saturating_add(1);
        }
    }
    CommitTally {
        commits_30d,
        distinct_contributors_90d: authors.len() as i32,
    }
}

fn commit_summary_from(commit: &octocrab::models::repos::RepoCommit) -> CommitSummary {
    let inner_author = commit.commit.author.as_ref();
    let author_key = commit
        .author
        .as_ref()
        .map(|a| a.login.clone())
        .filter(|s: &String| !s.is_empty())
        .or_else(|| {
            inner_author
                .and_then(|a| a.email.clone())
                .filter(|s: &String| !s.is_empty())
        })
        .or_else(|| {
            inner_author
                .map(|a| a.name.clone())
                .filter(|s: &String| !s.is_empty())
        })
        .unwrap_or_else(|| format!("sha:{}", commit.sha));
    let committed_at = commit
        .commit
        .author
        .as_ref()
        .and_then(|a| a.date)
        .or_else(|| commit.commit.committer.as_ref().and_then(|a| a.date))
        .unwrap_or_else(Utc::now);
    CommitSummary {
        author_key,
        committed_at,
    }
}

async fn fetch_structural_signals(client: &Octocrab, owner: &str, name: &str) -> StructuralSignals {
    let now = Utc::now();
    let cutoff_90d = now - Duration::days(COMMITS_LOOKBACK_DAYS);
    let cutoff_30d = now - Duration::days(COMMITS_30D_WINDOW);

    let (distinct_contributors_90d, commits_30d) =
        match fetch_commits_since(client, owner, name, cutoff_90d).await {
            Ok(commits) => {
                let tally = tally_commits(&commits, cutoff_30d);
                (
                    Some(tally.distinct_contributors_90d),
                    Some(tally.commits_30d),
                )
            }
            Err(err) => {
                tracing::warn!(
                    target: "ingestion::github::structural",
                    "commits fetch failed for {owner}/{name}: {err}"
                );
                (None, None)
            }
        };

    let has_ci = match fetch_has_ci(client, owner, name).await {
        Ok(value) => Some(value),
        Err(err) => {
            tracing::warn!(
                target: "ingestion::github::structural",
                "has_ci fetch failed for {owner}/{name}: {err}"
            );
            None
        }
    };

    let (releases_count, last_release_at) = match fetch_releases_summary(client, owner, name).await
    {
        Ok((count, last)) => (Some(count), last),
        Err(err) => {
            tracing::warn!(
                target: "ingestion::github::structural",
                "releases fetch failed for {owner}/{name}: {err}"
            );
            (None, None)
        }
    };

    StructuralSignals {
        distinct_contributors_90d,
        commits_30d,
        has_ci,
        releases_count,
        last_release_at,
        captured_at: Some(now),
    }
}

async fn fetch_commits_since(
    client: &Octocrab,
    owner: &str,
    name: &str,
    since: DateTime<Utc>,
) -> Result<Vec<CommitSummary>, octocrab::Error> {
    let mut summaries: Vec<CommitSummary> = Vec::new();
    let mut page = client
        .repos(owner, name)
        .list_commits()
        .since(since)
        .per_page(COMMITS_PER_PAGE)
        .send()
        .await?;
    let mut pages_remaining = COMMITS_MAX_PAGES;
    loop {
        for commit in &page.items {
            summaries.push(commit_summary_from(commit));
        }
        pages_remaining = pages_remaining.saturating_sub(1);
        if pages_remaining == 0 {
            break;
        }
        match client
            .get_page::<octocrab::models::repos::RepoCommit>(&page.next)
            .await?
        {
            Some(next) => page = next,
            None => break,
        }
    }
    Ok(summaries)
}

async fn fetch_has_ci(client: &Octocrab, owner: &str, name: &str) -> Result<bool, octocrab::Error> {
    match client
        .repos(owner, name)
        .get_content()
        .path(".github/workflows")
        .send()
        .await
    {
        Ok(content) => Ok(!content.items.is_empty()),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            Ok(false)
        }
        Err(err) => Err(err),
    }
}

async fn fetch_releases_summary(
    client: &Octocrab,
    owner: &str,
    name: &str,
) -> Result<(i32, Option<DateTime<Utc>>), octocrab::Error> {
    let page = client
        .repos(owner, name)
        .releases()
        .list()
        .per_page(100)
        .send()
        .await?;
    let count = page.items.len() as i32;
    let last = page.items.iter().filter_map(|r| r.published_at).max();
    Ok((count, last))
}

async fn fetch_readme_for_classification(
    client: &Octocrab,
    owner: &str,
    name: &str,
) -> Option<String> {
    match fetch_readme_text(client, owner, name).await {
        Ok(readme) => readme,
        Err(err) => {
            tracing::warn!(
                target: "ingestion::github::readme",
                "README fetch failed for {owner}/{name}: {err:?}"
            );
            None
        }
    }
}

async fn fetch_readme_text(
    client: &Octocrab,
    owner: &str,
    name: &str,
) -> Result<Option<String>, ApiError> {
    let path = format!("/repos/{owner}/{name}/readme");
    let response: GitHubReadmeResponse = match client.get(path, None::<&()>).await {
        Ok(response) => response,
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            return Ok(None);
        }
        Err(octocrab::Error::GitHub { source, .. })
            if source.status_code.as_u16() == 403 || source.status_code.as_u16() == 429 =>
        {
            return Err(ApiError::forbidden(
                "GitHub README fetch rate limited or denied",
            ));
        }
        Err(other) => {
            return Err(ApiError::internal(format!(
                "GitHub README fetch failed: {other}"
            )));
        }
    };

    let Some(content) = response.content else {
        return Ok(None);
    };
    let encoding = response.encoding.as_deref().unwrap_or("base64");
    decode_readme_content(&content, encoding).map(Some)
}

fn decode_readme_content(content: &str, encoding: &str) -> Result<String, ApiError> {
    let bytes = match encoding {
        "base64" => {
            let compact = content.split_whitespace().collect::<String>();
            BASE64_STANDARD
                .decode(compact.as_bytes())
                .map_err(|err| ApiError::internal(format!("README base64 decode failed: {err}")))?
        }
        "plain" => content.as_bytes().to_vec(),
        other => {
            return Err(ApiError::internal(format!(
                "unsupported README encoding: {other}"
            )));
        }
    };

    let limited = if bytes.len() > README_CLASSIFICATION_MAX_BYTES {
        &bytes[..README_CLASSIFICATION_MAX_BYTES]
    } else {
        &bytes
    };
    Ok(String::from_utf8_lossy(limited).to_string())
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
          last_commit_at, priors_fetched_at,
          distinct_contributors_90d, commits_30d, has_ci,
          releases_count, last_release_at, structural_signals_at
        )
        VALUES (
          CAST($1 AS external_source), $2, $3,
          $4, $5, $6,
          $7, $8, $9, $10,
          $11, $12, $13,
          $14, $15, $16, $17,
          $18, NOW(),
          $19, $20, $21,
          $22, $23, $24
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
          structural_signals_at    = COALESCE(EXCLUDED.structural_signals_at,    external_artifacts.structural_signals_at)
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
    .fetch_one(db)
    .await?;

    Ok(id)
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
    let meta = fetch_repo(client, owner, name).await?;
    let id = upsert_github_artifact(db, &meta).await?;
    let readme = fetch_readme_for_classification(client, &meta.owner, &meta.name).await;
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

pub fn parse_github_repo_input(input: &str) -> Result<(String, String), ApiError> {
    let trimmed = input.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return Err(ApiError::bad_request(
            "repo is required (GitHub URL or owner/repo)",
        ));
    }

    let candidate = trimmed
        .strip_prefix("https://github.com/")
        .or_else(|| trimmed.strip_prefix("http://github.com/"))
        .or_else(|| trimmed.strip_prefix("github.com/"))
        .unwrap_or(trimmed)
        .split(['?', '#'])
        .next()
        .unwrap_or(trimmed)
        .trim_end_matches(".git");

    let mut parts = candidate.split('/').filter(|p| !p.trim().is_empty());
    let owner = parts
        .next()
        .ok_or_else(|| ApiError::bad_request("repo must include owner/repo"))?;
    let name = parts
        .next()
        .ok_or_else(|| ApiError::bad_request("repo must include owner/repo"))?;

    if parts.next().is_some() {
        return Err(ApiError::bad_request(
            "repo must be a GitHub URL or owner/repo",
        ));
    }
    if owner.contains(' ') || name.contains(' ') {
        return Err(ApiError::bad_request("repo must not contain whitespace"));
    }

    Ok((owner.to_string(), name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{CommitSummary, parse_github_repo_input, tally_commits};
    use chrono::{Duration, Utc};

    #[test]
    fn parses_owner_repo() {
        let (owner, repo) = parse_github_repo_input("openai/gpt").unwrap();
        assert_eq!(owner, "openai");
        assert_eq!(repo, "gpt");
    }

    #[test]
    fn parses_url_with_query_and_git_suffix() {
        let (owner, repo) =
            parse_github_repo_input("https://github.com/openai/gpt.git?tab=readme").unwrap();
        assert_eq!(owner, "openai");
        assert_eq!(repo, "gpt");
    }

    #[test]
    fn decodes_base64_readme_content_for_classification() {
        let decoded = super::decode_readme_content("IyBLaXQgVUkK", "base64").unwrap();

        assert_eq!(decoded, "# Kit UI\n");
    }

    #[test]
    fn limits_readme_text_used_for_classification() {
        let oversized = "a".repeat(super::README_CLASSIFICATION_MAX_BYTES + 20);
        let decoded = super::decode_readme_content(&oversized, "plain").unwrap();

        assert_eq!(decoded.len(), super::README_CLASSIFICATION_MAX_BYTES);
    }

    #[test]
    fn rejects_extra_segments() {
        assert!(parse_github_repo_input("openai/gpt/issues").is_err());
    }

    #[test]
    fn tally_counts_distinct_authors_and_30d_window() {
        let now = Utc::now();
        let cutoff_30d = now - Duration::days(30);
        let commits = vec![
            CommitSummary {
                author_key: "alice".into(),
                committed_at: now - Duration::days(1),
            },
            CommitSummary {
                author_key: "alice".into(),
                committed_at: now - Duration::days(40),
            },
            CommitSummary {
                author_key: "bob".into(),
                committed_at: now - Duration::days(15),
            },
            CommitSummary {
                author_key: "carol".into(),
                committed_at: now - Duration::days(80),
            },
        ];
        let tally = tally_commits(&commits, cutoff_30d);
        assert_eq!(tally.distinct_contributors_90d, 3);
        assert_eq!(tally.commits_30d, 2);
    }

    #[test]
    fn tally_handles_empty_input() {
        let now = Utc::now();
        let cutoff_30d = now - Duration::days(30);
        let tally = tally_commits(&[], cutoff_30d);
        assert_eq!(tally.distinct_contributors_90d, 0);
        assert_eq!(tally.commits_30d, 0);
    }

    #[test]
    fn tally_solo_dev_high_cadence_is_one_contributor() {
        let now = Utc::now();
        let cutoff_30d = now - Duration::days(30);
        let commits: Vec<CommitSummary> = (0..50)
            .map(|i| CommitSummary {
                author_key: "solo-vibe-coder".into(),
                committed_at: now - Duration::hours(i),
            })
            .collect();
        let tally = tally_commits(&commits, cutoff_30d);
        assert_eq!(tally.distinct_contributors_90d, 1);
        assert_eq!(tally.commits_30d, 50);
    }

    #[test]
    fn tally_30d_boundary_is_inclusive() {
        let now = Utc::now();
        let cutoff_30d = now - Duration::days(30);
        let commits = vec![CommitSummary {
            author_key: "edge".into(),
            committed_at: cutoff_30d,
        }];
        let tally = tally_commits(&commits, cutoff_30d);
        assert_eq!(tally.commits_30d, 1);
    }
}
