use chrono::{DateTime, Utc};
use http::StatusCode;
use octocrab::Octocrab;

use super::client::github_api_failure;
use super::structural::{
    ExistingGitHubIngestionState, GitHubIngestionMetadata, StructuralSignals,
    fetch_latest_default_branch_commit_at, fetch_structural_signals,
};
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
    pub structural: StructuralSignals,
    pub ingestion: GitHubIngestionMetadata,
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

pub(crate) async fn fetch_repo_with_state(
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
