use std::collections::HashSet;
use std::time::Duration as StdDuration;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use chrono::{DateTime, Duration, Utc};
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{ETAG, IF_NONE_MATCH},
};
use http_body_util::BodyExt;
use octocrab::Octocrab;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::structural_extras;

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
const GITHUB_SECONDARY_RATE_LIMIT_MARKER: &str = "secondary rate limit";
const SECONDARY_RATE_LIMIT_DEFAULT_BACKOFF_SECS: u64 = 2;
const MAX_INLINE_BACKOFF_SECS: u64 = 2;

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
    pub ingestion: GitHubIngestionMetadata,
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
    pub owner_last_activity_at: Option<DateTime<Utc>>,
    pub owner_inactive_days: Option<i32>,
    pub releases_etag: Option<String>,
    pub events_etag: Option<String>,
}

#[cfg(test)]
impl StructuralSignals {
    fn merge_releases_not_modified(self) -> Self {
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct GitHubIngestionMetadata {
    pub releases_etag: Option<String>,
    pub readme_etag: Option<String>,
    pub events_etag: Option<String>,
    pub rate_limit_reset_at: Option<DateTime<Utc>>,
    pub last_rate_limit_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, FromRow)]
struct ExistingGitHubIngestionState {
    #[allow(dead_code)]
    repo_etag: Option<String>,
    releases_etag: Option<String>,
    readme_etag: Option<String>,
    events_etag: Option<String>,
    releases_count: Option<i32>,
    last_release_at: Option<DateTime<Utc>>,
    owner_last_activity_at: Option<DateTime<Utc>>,
    owner_inactive_days: Option<i32>,
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
    fetch_repo_with_state(
        client,
        owner,
        name,
        &ExistingGitHubIngestionState::default(),
    )
    .await
}

async fn fetch_repo_with_state(
    client: &Octocrab,
    owner: &str,
    name: &str,
    existing: &ExistingGitHubIngestionState,
) -> Result<GitHubRepoMetadata, ApiError> {
    let repo = client.repos(owner, name).get().await.map_err(|e| match e {
        octocrab::Error::GitHub { source, .. } if source.status_code.as_u16() == 404 => {
            ApiError::not_found(format!("github repo not found: {owner}/{name}"))
        }
        octocrab::Error::GitHub { source, .. }
            if source.status_code == StatusCode::FORBIDDEN
                || source.status_code == StatusCode::TOO_MANY_REQUESTS =>
        {
            github_api_failure("GitHub repo fetch", source.status_code, &source.message)
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

    let structural = fetch_structural_signals(client, &resolved_owner, &repo.name, existing).await;
    let ingestion = GitHubIngestionMetadata {
        releases_etag: structural.releases_etag.clone(),
        events_etag: structural.events_etag.clone(),
        ..Default::default()
    };

    // `pushed_at` updates on any branch/tag push; users expect the default-branch HEAD date.
    let last_commit_at = fetch_latest_default_branch_commit_at(
        client,
        &resolved_owner,
        &repo.name,
        repo.default_branch.as_deref(),
    )
    .await
    .or(repo.pushed_at);

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
        last_commit_at,
        structural,
        ingestion,
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum GitHubRateLimitKind {
    Primary {
        reset_at: Option<DateTime<Utc>>,
        retry_after: Option<StdDuration>,
    },
    Secondary,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GitHubReleaseSummary {
    published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct GitHubRepoEvent {
    #[serde(rename = "type")]
    event_type: String,
    actor: GitHubEventActor,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct GitHubEventActor {
    login: String,
}

pub(crate) struct GitHubJsonResponse<T> {
    pub(crate) data: Option<T>,
    pub(crate) etag: Option<String>,
    pub(crate) not_modified: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct OwnerActivitySummary {
    owner_last_activity_at: Option<DateTime<Utc>>,
    owner_inactive_days: Option<i32>,
}

struct OwnerActivityFetch {
    summary: OwnerActivitySummary,
    etag: Option<String>,
}

fn classify_rate_limit(
    status: StatusCode,
    headers: &HeaderMap,
    body: &str,
) -> Option<GitHubRateLimitKind> {
    if status != StatusCode::FORBIDDEN && status != StatusCode::TOO_MANY_REQUESTS {
        return None;
    }

    let body_lower = body.to_ascii_lowercase();
    if body_lower.contains(GITHUB_SECONDARY_RATE_LIMIT_MARKER) {
        return Some(GitHubRateLimitKind::Secondary);
    }

    let remaining = header_str(headers, "x-ratelimit-remaining");
    if remaining == Some("0") || status == StatusCode::TOO_MANY_REQUESTS {
        return Some(GitHubRateLimitKind::Primary {
            reset_at: header_str(headers, "x-ratelimit-reset")
                .and_then(|value| value.parse::<i64>().ok())
                .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0)),
            retry_after: header_str(headers, "retry-after")
                .and_then(|value| value.parse::<u64>().ok())
                .map(StdDuration::from_secs),
        });
    }

    None
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

pub(crate) fn summarize_releases(
    releases: &[GitHubReleaseSummary],
) -> (i32, Option<DateTime<Utc>>) {
    let count = releases.len() as i32;
    let last = releases
        .iter()
        .filter_map(|release| release.published_at)
        .max();
    (count, last)
}

fn conditional_request_headers(etag: Option<&str>) -> Option<HeaderMap> {
    let etag = etag?.trim();
    if etag.is_empty() {
        return None;
    }

    let value = HeaderValue::from_str(etag).ok()?;
    let mut headers = HeaderMap::new();
    headers.insert(IF_NONE_MATCH, value);
    Some(headers)
}

fn github_rate_limit_message(kind: &GitHubRateLimitKind) -> String {
    match kind {
        GitHubRateLimitKind::Primary {
            reset_at,
            retry_after,
        } => {
            let mut message = "GitHub API primary rate limit reached".to_string();
            if let Some(reset_at) = reset_at {
                message.push_str(&format!("; resets at {}", reset_at.to_rfc3339()));
            }
            if let Some(retry_after) = retry_after {
                message.push_str(&format!("; retry after {} seconds", retry_after.as_secs()));
            }
            message
        }
        GitHubRateLimitKind::Secondary => {
            "GitHub API secondary rate limit reached; retry after a short backoff".to_string()
        }
    }
}

fn retry_delay(limit: &Option<GitHubRateLimitKind>) -> Option<StdDuration> {
    match limit {
        Some(GitHubRateLimitKind::Primary {
            retry_after: Some(retry_after),
            ..
        }) => Some(*retry_after),
        Some(GitHubRateLimitKind::Secondary) => Some(StdDuration::from_secs(
            SECONDARY_RATE_LIMIT_DEFAULT_BACKOFF_SECS,
        )),
        _ => None,
    }
}

fn github_api_failure_with_headers(
    context: &str,
    status: StatusCode,
    headers: &HeaderMap,
    message: &str,
) -> ApiError {
    if let Some(kind) = classify_rate_limit(status, headers, message) {
        return ApiError::forbidden(format!("{context}: {}", github_rate_limit_message(&kind)));
    }

    github_api_failure(context, status, message)
}

fn github_api_failure(context: &str, status: StatusCode, message: &str) -> ApiError {
    let headers = HeaderMap::new();
    if let Some(kind) = classify_rate_limit(status, &headers, message) {
        return ApiError::forbidden(format!("{context}: {}", github_rate_limit_message(&kind)));
    }

    match status {
        StatusCode::FORBIDDEN => {
            ApiError::forbidden(format!("{context}: GitHub returned 403 ({message})"))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            ApiError::forbidden(format!("{context}: GitHub returned 429 ({message})"))
        }
        _ => ApiError::internal(format!(
            "{context}: GitHub returned {} ({message})",
            status.as_u16()
        )),
    }
}

pub(crate) async fn github_get_json_with_etag<T>(
    client: &Octocrab,
    path: &str,
    etag: Option<&str>,
    context: &str,
) -> Result<GitHubJsonResponse<T>, ApiError>
where
    T: for<'de> Deserialize<'de>,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        let response = client
            ._get_with_headers(path, conditional_request_headers(etag))
            .await
            .map_err(|err| ApiError::internal(format!("{context} failed: {err}")))?;
        let status = response.status();
        let headers = response.headers().clone();
        let response_etag = header_str(&headers, ETAG.as_str()).map(str::to_string);

        if status == StatusCode::NOT_MODIFIED {
            return Ok(GitHubJsonResponse {
                data: None,
                etag: response_etag.or_else(|| etag.map(str::to_string)),
                not_modified: true,
            });
        }

        let body = response
            .into_body()
            .collect()
            .await
            .map_err(|err| ApiError::internal(format!("{context} body read failed: {err}")))?
            .to_bytes();
        let body_text = String::from_utf8_lossy(&body);
        let rate_limit = classify_rate_limit(status, &headers, &body_text);
        if rate_limit.is_some() {
            if attempts == 1
                && let Some(delay) = retry_delay(&rate_limit)
                && delay.as_secs() <= MAX_INLINE_BACKOFF_SECS
            {
                tokio::time::sleep(delay).await;
                continue;
            }
            return Err(github_api_failure_with_headers(
                context, status, &headers, &body_text,
            ));
        }
        if !status.is_success() {
            if status == StatusCode::NOT_FOUND {
                return Err(ApiError::not_found(format!(
                    "{context}: GitHub returned 404"
                )));
            }
            return Err(github_api_failure_with_headers(
                context, status, &headers, &body_text,
            ));
        }

        let data = serde_json::from_slice::<T>(&body)
            .map_err(|err| ApiError::internal(format!("{context} JSON decode failed: {err}")))?;
        return Ok(GitHubJsonResponse {
            data: Some(data),
            etag: response_etag,
            not_modified: false,
        });
    }
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

async fn fetch_structural_signals(
    client: &Octocrab,
    owner: &str,
    name: &str,
    existing: &ExistingGitHubIngestionState,
) -> StructuralSignals {
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

    let has_ci = structural_extras::detect_has_ci(client, owner, name).await;

    let (releases_count, last_release_at, releases_etag) =
        match structural_extras::fetch_releases_with_fallback(
            client,
            owner,
            name,
            existing.releases_etag.as_deref(),
            existing.releases_count,
            existing.last_release_at,
        )
        .await
        {
            Ok(summary) => (
                summary.releases_count,
                summary.last_release_at,
                summary.etag,
            ),
            Err(err) => {
                tracing::warn!(
                    target: "ingestion::github::structural",
                    "releases fetch failed for {owner}/{name}: {}",
                    err.message
                );
                (None, None, existing.releases_etag.clone())
            }
        };

    let owner_activity = match fetch_owner_activity_summary(
        client,
        owner,
        name,
        existing.events_etag.as_deref(),
        now,
        existing.owner_last_activity_at,
        existing.owner_inactive_days,
    )
    .await
    {
        Ok(activity) => activity,
        Err(err) => {
            tracing::warn!(
                target: "ingestion::github::structural",
                "events fetch failed for {owner}/{name}: {}",
                err.message
            );
            OwnerActivityFetch {
                summary: OwnerActivitySummary {
                    owner_last_activity_at: existing.owner_last_activity_at,
                    owner_inactive_days: existing.owner_inactive_days,
                },
                etag: existing.events_etag.clone(),
            }
        }
    };

    StructuralSignals {
        distinct_contributors_90d,
        commits_30d,
        has_ci,
        releases_count,
        last_release_at,
        captured_at: Some(now),
        owner_last_activity_at: owner_activity.summary.owner_last_activity_at,
        owner_inactive_days: owner_activity.summary.owner_inactive_days,
        releases_etag,
        events_etag: owner_activity.etag,
    }
}

async fn fetch_latest_default_branch_commit_at(
    client: &Octocrab,
    owner: &str,
    name: &str,
    default_branch: Option<&str>,
) -> Option<DateTime<Utc>> {
    let repo = client.repos(owner, name);
    let mut request = repo.list_commits().per_page(1);
    if let Some(branch) = default_branch.filter(|branch| !branch.is_empty()) {
        request = request.sha(branch);
    }

    match request.send().await {
        Ok(page) => page
            .items
            .first()
            .map(|commit| commit_summary_from(commit).committed_at),
        Err(err) => {
            tracing::warn!(
                target: "ingestion::github",
                "latest default-branch commit fetch failed for {owner}/{name}: {err}"
            );
            None
        }
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

struct ReadmeFetch {
    content: Option<String>,
    etag: Option<String>,
}

async fn fetch_readme_text_with_etag(
    client: &Octocrab,
    owner: &str,
    name: &str,
    etag: Option<&str>,
) -> Result<ReadmeFetch, ApiError> {
    let path = format!("/repos/{owner}/{name}/readme");
    let response: GitHubJsonResponse<GitHubReadmeResponse> =
        match github_get_json_with_etag(client, &path, etag, "GitHub README fetch").await {
            Ok(response) => response,
            Err(err) if err.status == StatusCode::NOT_FOUND => {
                return Ok(ReadmeFetch {
                    content: None,
                    etag: None,
                });
            }
            Err(err) => return Err(err),
        };

    if response.not_modified {
        return Ok(ReadmeFetch {
            content: None,
            etag: response.etag,
        });
    }

    let Some(response_data) = response.data else {
        return Ok(ReadmeFetch {
            content: None,
            etag: response.etag,
        });
    };
    let Some(content) = response_data.content else {
        return Ok(ReadmeFetch {
            content: None,
            etag: response.etag,
        });
    };
    let encoding = response_data.encoding.as_deref().unwrap_or("base64");
    decode_readme_content(&content, encoding).map(|content| ReadmeFetch {
        content: Some(content),
        etag: response.etag,
    })
}

async fn fetch_owner_activity_summary(
    client: &Octocrab,
    owner: &str,
    name: &str,
    etag: Option<&str>,
    now: DateTime<Utc>,
    existing_last_activity_at: Option<DateTime<Utc>>,
    existing_inactive_days: Option<i32>,
) -> Result<OwnerActivityFetch, ApiError> {
    let path = format!("/repos/{owner}/{name}/events?per_page=100");
    let response: GitHubJsonResponse<Vec<GitHubRepoEvent>> =
        github_get_json_with_etag(client, &path, etag, "GitHub events fetch").await?;
    let summary = if response.not_modified {
        OwnerActivitySummary {
            owner_last_activity_at: existing_last_activity_at,
            owner_inactive_days: existing_inactive_days,
        }
    } else {
        owner_activity_summary(&response.data.unwrap_or_default(), owner, now)
    };
    Ok(OwnerActivityFetch {
        summary,
        etag: response.etag,
    })
}

fn owner_activity_summary(
    events: &[GitHubRepoEvent],
    owner: &str,
    now: DateTime<Utc>,
) -> OwnerActivitySummary {
    let owner_lower = owner.to_ascii_lowercase();
    let last = events
        .iter()
        .filter(|event| is_maintainer_activity(event, &owner_lower))
        .map(|event| event.created_at)
        .max();
    let owner_inactive_days = last.map(|last| {
        now.signed_duration_since(last)
            .num_days()
            .max(0)
            .try_into()
            .unwrap_or(i32::MAX)
    });
    OwnerActivitySummary {
        owner_last_activity_at: last,
        owner_inactive_days,
    }
}

fn is_maintainer_activity(event: &GitHubRepoEvent, owner_lower: &str) -> bool {
    let actor = event.actor.login.to_ascii_lowercase();
    if actor.ends_with("[bot]") {
        return false;
    }
    if actor == owner_lower {
        return true;
    }
    matches!(
        event.event_type.as_str(),
        "PushEvent" | "ReleaseEvent" | "PullRequestEvent" | "IssuesEvent"
    )
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

async fn load_existing_ingestion_state(
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
    use chrono::{Duration, TimeZone, Utc};
    use http::{HeaderMap, HeaderValue, StatusCode, header::IF_NONE_MATCH};

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

    #[test]
    fn github_rate_limit_headers_detect_primary_limit() {
        let headers = github_rate_limit_headers("0", "1778791697", None);
        let limit = super::classify_rate_limit(StatusCode::FORBIDDEN, &headers, "");

        assert!(matches!(
            limit,
            Some(super::GitHubRateLimitKind::Primary { .. })
        ));
    }

    #[test]
    fn github_rate_limit_body_detects_secondary_limit() {
        let headers = github_rate_limit_headers("42", "1778791697", None);
        let limit = super::classify_rate_limit(
            StatusCode::FORBIDDEN,
            &headers,
            "You have exceeded a secondary rate limit. Please wait a few minutes.",
        );

        assert!(matches!(limit, Some(super::GitHubRateLimitKind::Secondary)));
    }

    #[test]
    fn github_api_failure_maps_secondary_limit_with_context() {
        let err = super::github_api_failure(
            "GitHub releases fetch",
            StatusCode::FORBIDDEN,
            "You have exceeded a secondary rate limit.",
        );

        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert!(err.message.contains("GitHub releases fetch"));
        assert!(err.message.contains("secondary rate limit"));
    }

    #[test]
    fn github_api_failure_maps_access_denied_with_status_context() {
        let err = super::github_api_failure(
            "GitHub releases fetch",
            StatusCode::FORBIDDEN,
            "Resource not accessible by integration",
        );

        assert_eq!(err.status, StatusCode::FORBIDDEN);
        assert!(err.message.contains("GitHub releases fetch"));
        assert!(err.message.contains("403"));
    }

    #[test]
    fn release_summary_selects_newest_published_release() {
        let old = Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap();
        let new = Utc.with_ymd_and_hms(2026, 5, 2, 12, 0, 0).unwrap();
        let releases = vec![
            super::GitHubReleaseSummary {
                published_at: Some(old),
            },
            super::GitHubReleaseSummary {
                published_at: Some(new),
            },
            super::GitHubReleaseSummary { published_at: None },
        ];

        let (count, last_release_at) = super::summarize_releases(&releases);

        assert_eq!(count, 3);
        assert_eq!(last_release_at, Some(new));
    }

    #[test]
    fn release_summary_handles_empty_releases() {
        let (count, last_release_at) = super::summarize_releases(&[]);

        assert_eq!(count, 0);
        assert_eq!(last_release_at, None);
    }

    #[test]
    fn conditional_headers_include_etag_when_present() {
        let headers = super::conditional_request_headers(Some(r#""abc123""#))
            .expect("etag should build headers");

        assert_eq!(
            headers
                .get(IF_NONE_MATCH)
                .and_then(|value| value.to_str().ok()),
            Some(r#""abc123""#)
        );
    }

    #[test]
    fn conditional_headers_skip_blank_etag() {
        assert!(super::conditional_request_headers(Some("   ")).is_none());
        assert!(super::conditional_request_headers(None).is_none());
    }

    #[test]
    fn etag_not_modified_preserves_existing_release_values() {
        let existing = super::StructuralSignals {
            releases_count: Some(4),
            last_release_at: Some(parse_dt("2026-05-01T00:00:00Z")),
            ..Default::default()
        };

        let refreshed = existing.clone().merge_releases_not_modified();

        assert_eq!(refreshed.releases_count, existing.releases_count);
        assert_eq!(refreshed.last_release_at, existing.last_release_at);
    }

    #[test]
    fn backoff_delay_uses_retry_after_before_default_secondary_delay() {
        let headers = github_rate_limit_headers("12", "1778791697", Some("7"));
        let limit = super::classify_rate_limit(StatusCode::TOO_MANY_REQUESTS, &headers, "");

        assert_eq!(super::retry_delay(&limit).map(|d| d.as_secs()), Some(7));
    }

    #[test]
    fn owner_activity_ignores_bots_and_counts_same_day_activity() {
        let now = parse_dt("2026-05-16T12:00:00Z");
        let events = vec![
            super::GitHubRepoEvent {
                event_type: "PushEvent".to_string(),
                actor: super::GitHubEventActor {
                    login: "dependabot[bot]".to_string(),
                },
                created_at: parse_dt("2026-05-16T11:00:00Z"),
            },
            super::GitHubRepoEvent {
                event_type: "IssuesEvent".to_string(),
                actor: super::GitHubEventActor {
                    login: "vitejs".to_string(),
                },
                created_at: parse_dt("2026-05-16T10:00:00Z"),
            },
        ];

        let activity = super::owner_activity_summary(&events, "vitejs", now);

        assert_eq!(
            activity.owner_last_activity_at,
            Some(parse_dt("2026-05-16T10:00:00Z"))
        );
        assert_eq!(activity.owner_inactive_days, Some(0));
    }

    #[test]
    fn owner_activity_returns_none_for_empty_events() {
        let activity =
            super::owner_activity_summary(&[], "vitejs", parse_dt("2026-05-16T12:00:00Z"));

        assert_eq!(activity.owner_last_activity_at, None);
        assert_eq!(activity.owner_inactive_days, None);
    }

    fn github_rate_limit_headers(
        remaining: &str,
        reset: &str,
        retry_after: Option<&str>,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-ratelimit-remaining",
            HeaderValue::from_str(remaining).unwrap(),
        );
        headers.insert("x-ratelimit-reset", HeaderValue::from_str(reset).unwrap());
        if let Some(retry_after) = retry_after {
            headers.insert("retry-after", HeaderValue::from_str(retry_after).unwrap());
        }
        headers
    }

    fn parse_dt(value: &str) -> chrono::DateTime<Utc> {
        value.parse().unwrap()
    }
}
