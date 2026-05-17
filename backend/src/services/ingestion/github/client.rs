use std::time::Duration as StdDuration;

use chrono::{DateTime, Utc};
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{ETAG, IF_NONE_MATCH},
};
use http_body_util::BodyExt;
use octocrab::Octocrab;
use serde::Deserialize;

use crate::{app::error::ApiError, services::ingestion::github_quota};

const GITHUB_SECONDARY_RATE_LIMIT_MARKER: &str = "secondary rate limit";
const SECONDARY_RATE_LIMIT_DEFAULT_BACKOFF_SECS: u64 = 2;
const MAX_INLINE_BACKOFF_SECS: u64 = 2;

pub fn build_client(token: &str) -> Result<Octocrab, ApiError> {
    Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .map_err(|e| ApiError::internal(format!("github client build failed: {e}")))
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GitHubRateLimitKind {
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

pub(crate) struct GitHubJsonResponse<T> {
    pub(crate) data: Option<T>,
    pub(crate) etag: Option<String>,
    pub(crate) not_modified: bool,
}

pub(crate) fn classify_rate_limit(
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

/// Structured quota snapshot for operators (see public-launch-hardening Task 3).
fn log_github_rate_limit_snapshot(context: &str, status: StatusCode, headers: &HeaderMap) {
    let remaining = header_str(headers, "x-ratelimit-remaining");
    let limit = header_str(headers, "x-ratelimit-limit");
    let reset = header_str(headers, "x-ratelimit-reset");
    let used = header_str(headers, "x-ratelimit-used");
    if remaining.is_none() && limit.is_none() {
        return;
    }

    let remaining_i64 = remaining.and_then(|value| value.parse::<i64>().ok());
    let fields = (context, status.as_u16(), remaining, limit, reset, used);

    if let Some(rem) = remaining_i64
        && rem <= 100
    {
        tracing::warn!(
            github_context = fields.0,
            http_status = fields.1,
            github_rate_limit_remaining = rem,
            github_rate_limit_limit = fields.3,
            github_rate_limit_reset = fields.4,
            github_rate_limit_used = fields.5,
            "GitHub API rate limit low"
        );
        return;
    }

    tracing::debug!(
        github_context = fields.0,
        http_status = fields.1,
        github_rate_limit_remaining = fields.2,
        github_rate_limit_limit = fields.3,
        github_rate_limit_reset = fields.4,
        github_rate_limit_used = fields.5,
        "GitHub API rate limit snapshot"
    );

    github_quota::record_headers_snapshot(context, headers);
}

fn log_github_rate_limit_hit(
    context: &str,
    status: StatusCode,
    headers: &HeaderMap,
    kind: &GitHubRateLimitKind,
) {
    log_github_rate_limit_snapshot(context, status, headers);
    match kind {
        GitHubRateLimitKind::Secondary => {
            github_quota::record_limit_hit("secondary");
            tracing::warn!(
                github_context = context,
                http_status = status.as_u16(),
                github_rate_limit_kind = "secondary",
                "GitHub API secondary rate limit"
            );
        }
        GitHubRateLimitKind::Primary {
            reset_at,
            retry_after,
        } => {
            github_quota::record_limit_hit("primary");
            tracing::warn!(
                github_context = context,
                http_status = status.as_u16(),
                github_rate_limit_kind = "primary",
                github_rate_limit_reset_at = ?reset_at,
                github_retry_after_secs = retry_after.map(|d| d.as_secs()),
                "GitHub API primary rate limit"
            );
        }
    }
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

pub(crate) fn github_api_failure_with_headers(
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

pub(crate) fn github_api_failure(context: &str, status: StatusCode, message: &str) -> ApiError {
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
        if let Some(ref kind) = rate_limit {
            log_github_rate_limit_hit(context, status, &headers, kind);
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
        log_github_rate_limit_snapshot(context, status, &headers);
        return Ok(GitHubJsonResponse {
            data: Some(data),
            etag: response_etag,
            not_modified: false,
        });
    }
}
#[cfg(test)]
mod tests {
    use super::{
        GitHubRateLimitKind, GitHubReleaseSummary, classify_rate_limit,
        conditional_request_headers, github_api_failure, retry_delay, summarize_releases,
    };
    use chrono::{TimeZone, Utc};
    use http::{HeaderMap, HeaderValue, StatusCode, header::IF_NONE_MATCH};

    #[test]
    fn github_rate_limit_headers_detect_primary_limit() {
        let headers = github_rate_limit_headers("0", "1778791697", None);
        let limit = classify_rate_limit(StatusCode::FORBIDDEN, &headers, "");

        assert!(matches!(limit, Some(GitHubRateLimitKind::Primary { .. })));
    }

    #[test]
    fn github_rate_limit_body_detects_secondary_limit() {
        let headers = github_rate_limit_headers("42", "1778791697", None);
        let limit = classify_rate_limit(
            StatusCode::FORBIDDEN,
            &headers,
            "You have exceeded a secondary rate limit. Please wait a few minutes.",
        );

        assert!(matches!(limit, Some(GitHubRateLimitKind::Secondary)));
    }

    #[test]
    fn github_api_failure_maps_secondary_limit_with_context() {
        let err = github_api_failure(
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
        let err = github_api_failure(
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
            GitHubReleaseSummary {
                published_at: Some(old),
            },
            GitHubReleaseSummary {
                published_at: Some(new),
            },
            GitHubReleaseSummary { published_at: None },
        ];

        let (count, last_release_at) = summarize_releases(&releases);

        assert_eq!(count, 3);
        assert_eq!(last_release_at, Some(new));
    }

    #[test]
    fn release_summary_handles_empty_releases() {
        let (count, last_release_at) = summarize_releases(&[]);

        assert_eq!(count, 0);
        assert_eq!(last_release_at, None);
    }

    #[test]
    fn conditional_headers_include_etag_when_present() {
        let headers =
            conditional_request_headers(Some(r#""abc123""#)).expect("etag should build headers");

        assert_eq!(
            headers
                .get(IF_NONE_MATCH)
                .and_then(|value| value.to_str().ok()),
            Some(r#""abc123""#)
        );
    }

    #[test]
    fn conditional_headers_skip_blank_etag() {
        assert!(conditional_request_headers(Some("   ")).is_none());
        assert!(conditional_request_headers(None).is_none());
    }

    #[test]
    fn backoff_delay_uses_retry_after_before_default_secondary_delay() {
        let headers = github_rate_limit_headers("12", "1778791697", Some("7"));
        let limit = classify_rate_limit(StatusCode::TOO_MANY_REQUESTS, &headers, "");

        assert_eq!(retry_delay(&limit).map(|d| d.as_secs()), Some(7));
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
}
