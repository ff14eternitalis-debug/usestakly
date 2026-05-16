use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde::Deserialize;

use crate::app::error::ApiError;

use super::github::{GitHubJsonResponse, github_get_json_with_etag, summarize_releases};

const RELEASES_PER_PAGE: u32 = 100;
const RELEASES_MAX_PAGES: u32 = 5;
const TAGS_PER_PAGE: u32 = 100;

#[derive(Debug, Clone)]
pub struct ReleaseFetchResult {
    pub releases_count: Option<i32>,
    pub last_release_at: Option<DateTime<Utc>>,
    pub etag: Option<String>,
}

pub async fn detect_has_ci(client: &Octocrab, owner: &str, name: &str) -> Option<bool> {
    if check_github_workflows(client, owner, name).await == Some(true) {
        return Some(true);
    }
    for path in [
        ".gitlab-ci.yml",
        "azure-pipelines.yml",
        ".circleci/config.yml",
    ] {
        if path_exists(client, owner, name, path).await == Some(true) {
            return Some(true);
        }
    }
    check_github_workflows(client, owner, name).await
}

async fn check_github_workflows(client: &Octocrab, owner: &str, name: &str) -> Option<bool> {
    match client
        .repos(owner, name)
        .get_content()
        .path(".github/workflows")
        .send()
        .await
    {
        Ok(content) => Some(!content.items.is_empty()),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            Some(false)
        }
        Err(_) => None,
    }
}

async fn path_exists(client: &Octocrab, owner: &str, name: &str, path: &str) -> Option<bool> {
    match client
        .repos(owner, name)
        .get_content()
        .path(path)
        .send()
        .await
    {
        Ok(_) => Some(true),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            Some(false)
        }
        Err(_) => None,
    }
}

pub async fn fetch_releases_with_fallback(
    client: &Octocrab,
    owner: &str,
    name: &str,
    etag: Option<&str>,
    existing_count: Option<i32>,
    existing_last: Option<DateTime<Utc>>,
) -> Result<ReleaseFetchResult, ApiError> {
    let result =
        fetch_releases_paginated(client, owner, name, etag, existing_count, existing_last).await?;
    if result.releases_count.unwrap_or(0) > 0 {
        return Ok(result);
    }
    let tags = fetch_tags_estimate(client, owner, name).await?;
    if tags.releases_count.unwrap_or(0) > 0 {
        Ok(tags)
    } else {
        Ok(result)
    }
}

async fn fetch_releases_paginated(
    client: &Octocrab,
    owner: &str,
    name: &str,
    etag: Option<&str>,
    existing_count: Option<i32>,
    existing_last: Option<DateTime<Utc>>,
) -> Result<ReleaseFetchResult, ApiError> {
    let mut all = Vec::new();
    let mut response_etag = None;
    for page in 1..=RELEASES_MAX_PAGES {
        let path = if page == 1 {
            format!("/repos/{owner}/{name}/releases?per_page={RELEASES_PER_PAGE}")
        } else {
            format!("/repos/{owner}/{name}/releases?per_page={RELEASES_PER_PAGE}&page={page}")
        };
        let response: GitHubJsonResponse<Vec<super::github::GitHubReleaseSummary>> =
            github_get_json_with_etag(
                client,
                &path,
                if page == 1 { etag } else { None },
                "GitHub releases fetch",
            )
            .await?;
        if page == 1 && response.not_modified {
            return Ok(ReleaseFetchResult {
                releases_count: existing_count,
                last_release_at: existing_last,
                etag: response.etag,
            });
        }
        if page == 1 {
            response_etag = response.etag;
        }
        let batch = response.data.unwrap_or_default();
        if batch.is_empty() {
            break;
        }
        let page_full = batch.len() >= RELEASES_PER_PAGE as usize;
        all.extend(batch);
        if !page_full {
            break;
        }
    }
    let (count, last) = summarize_releases(&all);
    Ok(ReleaseFetchResult {
        releases_count: Some(count),
        last_release_at: last,
        etag: response_etag,
    })
}

#[derive(Debug, Deserialize)]
struct GitHubTagSummary {
    #[allow(dead_code)]
    name: String,
}

async fn fetch_tags_estimate(
    client: &Octocrab,
    owner: &str,
    name: &str,
) -> Result<ReleaseFetchResult, ApiError> {
    let path = format!("/repos/{owner}/{name}/tags?per_page={TAGS_PER_PAGE}");
    let response: GitHubJsonResponse<Vec<GitHubTagSummary>> =
        github_get_json_with_etag(client, &path, None, "GitHub tags fetch").await?;
    let tags = response.data.unwrap_or_default();
    Ok(ReleaseFetchResult {
        releases_count: Some(tags.len() as i32),
        last_release_at: None,
        etag: None,
    })
}
