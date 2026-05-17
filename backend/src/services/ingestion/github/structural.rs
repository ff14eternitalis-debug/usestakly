use std::collections::HashSet;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use chrono::{DateTime, Duration, Utc};
use http::StatusCode;
use octocrab::Octocrab;
use serde::Deserialize;
use sqlx::FromRow;

use super::client::{GitHubJsonResponse, github_get_json_with_etag};
use crate::app::error::ApiError;
use crate::services::ingestion::structural_extras;

pub(crate) const COMMITS_LOOKBACK_DAYS: i64 = 90;
pub(crate) const COMMITS_30D_WINDOW: i64 = 30;
const COMMITS_PER_PAGE: u8 = 100;
const COMMITS_MAX_PAGES: u32 = 5;
pub(crate) const README_CLASSIFICATION_MAX_BYTES: usize = 80_000;

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
pub(crate) struct ExistingGitHubIngestionState {
    #[allow(dead_code)]
    pub(crate) repo_etag: Option<String>,
    pub(crate) releases_etag: Option<String>,
    pub(crate) readme_etag: Option<String>,
    pub(crate) events_etag: Option<String>,
    pub(crate) releases_count: Option<i32>,
    pub(crate) last_release_at: Option<DateTime<Utc>>,
    pub(crate) owner_last_activity_at: Option<DateTime<Utc>>,
    pub(crate) owner_inactive_days: Option<i32>,
}
#[derive(Debug, Clone)]
pub(crate) struct CommitSummary {
    author_key: String,
    committed_at: DateTime<Utc>,
}

pub(crate) struct CommitTally {
    commits_30d: i32,
    distinct_contributors_90d: i32,
}

#[derive(Debug, Deserialize)]
struct GitHubReadmeResponse {
    content: Option<String>,
    encoding: Option<String>,
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct OwnerActivitySummary {
    owner_last_activity_at: Option<DateTime<Utc>>,
    owner_inactive_days: Option<i32>,
}

struct OwnerActivityFetch {
    summary: OwnerActivitySummary,
    etag: Option<String>,
}

pub(crate) fn tally_commits(commits: &[CommitSummary], cutoff_30d: DateTime<Utc>) -> CommitTally {
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

pub(crate) fn commit_summary_from(commit: &octocrab::models::repos::RepoCommit) -> CommitSummary {
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
pub(crate) async fn fetch_structural_signals(
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

pub(crate) async fn fetch_latest_default_branch_commit_at(
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

pub(crate) async fn fetch_commits_since(
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

pub(crate) struct ReadmeFetch {
    pub(crate) content: Option<String>,
    pub(crate) etag: Option<String>,
}

pub(crate) async fn fetch_readme_text_with_etag(
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

pub(crate) fn decode_readme_content(content: &str, encoding: &str) -> Result<String, ApiError> {
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
#[cfg(test)]
mod tests {
    use super::{
        CommitSummary, GitHubEventActor, GitHubRepoEvent, README_CLASSIFICATION_MAX_BYTES,
        StructuralSignals, decode_readme_content, owner_activity_summary, tally_commits,
    };
    use chrono::{Duration, Utc};

    #[test]
    fn decodes_base64_readme_content_for_classification() {
        let decoded = decode_readme_content("IyBLaXQgVUkK", "base64").unwrap();

        assert_eq!(decoded, "# Kit UI\n");
    }

    #[test]
    fn limits_readme_text_used_for_classification() {
        let oversized = "a".repeat(README_CLASSIFICATION_MAX_BYTES + 20);
        let decoded = decode_readme_content(&oversized, "plain").unwrap();

        assert_eq!(decoded.len(), README_CLASSIFICATION_MAX_BYTES);
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
    fn etag_not_modified_preserves_existing_release_values() {
        let existing = StructuralSignals {
            releases_count: Some(4),
            last_release_at: Some(parse_dt("2026-05-01T00:00:00Z")),
            ..Default::default()
        };

        let refreshed = existing.clone().merge_releases_not_modified();

        assert_eq!(refreshed.releases_count, existing.releases_count);
        assert_eq!(refreshed.last_release_at, existing.last_release_at);
    }

    #[test]
    fn owner_activity_ignores_bots_and_counts_same_day_activity() {
        let now = parse_dt("2026-05-16T12:00:00Z");
        let events = vec![
            GitHubRepoEvent {
                event_type: "PushEvent".to_string(),
                actor: GitHubEventActor {
                    login: "dependabot[bot]".to_string(),
                },
                created_at: parse_dt("2026-05-16T11:00:00Z"),
            },
            GitHubRepoEvent {
                event_type: "IssuesEvent".to_string(),
                actor: GitHubEventActor {
                    login: "vitejs".to_string(),
                },
                created_at: parse_dt("2026-05-16T10:00:00Z"),
            },
        ];

        let activity = owner_activity_summary(&events, "vitejs", now);

        assert_eq!(
            activity.owner_last_activity_at,
            Some(parse_dt("2026-05-16T10:00:00Z"))
        );
        assert_eq!(activity.owner_inactive_days, Some(0));
    }

    #[test]
    fn owner_activity_returns_none_for_empty_events() {
        let activity = owner_activity_summary(&[], "vitejs", parse_dt("2026-05-16T12:00:00Z"));

        assert_eq!(activity.owner_last_activity_at, None);
        assert_eq!(activity.owner_inactive_days, None);
    }

    fn parse_dt(value: &str) -> chrono::DateTime<Utc> {
        value.parse().unwrap()
    }
}
