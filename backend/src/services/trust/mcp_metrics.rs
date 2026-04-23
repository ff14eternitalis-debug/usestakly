use serde::Serialize;
use sqlx::PgPool;

use crate::app::error::ApiError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricsWindow {
    Day,
    Week,
    Month,
}

impl MetricsWindow {
    pub fn as_interval(self) -> &'static str {
        match self {
            MetricsWindow::Day => "24 hours",
            MetricsWindow::Week => "7 days",
            MetricsWindow::Month => "30 days",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MetricsWindow::Day => "24h",
            MetricsWindow::Week => "7d",
            MetricsWindow::Month => "30d",
        }
    }

    pub fn parse(raw: Option<&str>) -> Result<Self, ApiError> {
        match raw.map(|s| s.trim().to_ascii_lowercase()).as_deref() {
            None | Some("") | Some("7d") | Some("week") => Ok(MetricsWindow::Week),
            Some("24h") | Some("day") | Some("1d") => Ok(MetricsWindow::Day),
            Some("30d") | Some("month") => Ok(MetricsWindow::Month),
            Some(other) => Err(ApiError::bad_request(format!(
                "invalid window '{other}' (expected 24h, 7d or 30d)"
            ))),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpMetricsReport {
    pub window: String,
    pub totals: McpTotals,
    pub outcome_distribution: Vec<OutcomeBucket>,
    pub rejection_breakdown: Vec<RejectionBucket>,
    pub top_repos: Vec<RepoVolume>,
    pub top_users: Vec<UserVolume>,
    pub daily_volume: Vec<DailyBucket>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpTotals {
    pub log_usage: i64,
    pub watch_repo: i64,
    pub rejections: i64,
    pub distinct_tokens: i64,
    pub distinct_users: i64,
    pub distinct_repos: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomeBucket {
    pub outcome: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RejectionBucket {
    pub tool: String,
    pub reason: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoVolume {
    pub owner: String,
    pub name: String,
    pub log_usage: i64,
    pub watch_repo: i64,
    pub rejections: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserVolume {
    pub user_id: String,
    pub log_usage: i64,
    pub watch_repo: i64,
    pub rejections: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyBucket {
    pub bucket: String,
    pub log_usage: i64,
    pub watch_repo: i64,
    pub rejections: i64,
}

pub async fn gather_metrics(
    db: &PgPool,
    window: MetricsWindow,
) -> Result<McpMetricsReport, ApiError> {
    let interval = window.as_interval();
    let totals = fetch_totals(db, interval).await?;
    let outcome_distribution = fetch_outcome_distribution(db, interval).await?;
    let rejection_breakdown = fetch_rejection_breakdown(db, interval).await?;
    let top_repos = fetch_top_repos(db, interval).await?;
    let top_users = fetch_top_users(db, interval).await?;
    let daily_volume = fetch_daily_volume(db, interval).await?;

    Ok(McpMetricsReport {
        window: window.as_str().to_string(),
        totals,
        outcome_distribution,
        rejection_breakdown,
        top_repos,
        top_users,
        daily_volume,
    })
}

async fn fetch_totals(db: &PgPool, interval: &str) -> Result<McpTotals, ApiError> {
    let row: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(&format!(
        r#"
        SELECT
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage') AS log_usage,
          COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo') AS watch_repo,
          COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection') AS rejections,
          COUNT(DISTINCT token_id) AS distinct_tokens,
          COUNT(DISTINCT user_id) AS distinct_users,
          COUNT(DISTINCT (lower(repo_owner), lower(repo_name)))
            FILTER (WHERE repo_owner IS NOT NULL AND repo_name IS NOT NULL) AS distinct_repos
        FROM agent_token_events
        WHERE created_at >= NOW() - INTERVAL '{interval}'
        "#
    ))
    .fetch_one(db)
    .await?;

    Ok(McpTotals {
        log_usage: row.0,
        watch_repo: row.1,
        rejections: row.2,
        distinct_tokens: row.3,
        distinct_users: row.4,
        distinct_repos: row.5,
    })
}

async fn fetch_outcome_distribution(
    db: &PgPool,
    interval: &str,
) -> Result<Vec<OutcomeBucket>, ApiError> {
    let rows: Vec<(String, i64)> = sqlx::query_as(&format!(
        r#"
        SELECT
          COALESCE(payload->>'outcome', 'unknown') AS outcome,
          COUNT(*) AS count
        FROM agent_token_events
        WHERE kind = 'mcp_log_usage'
          AND created_at >= NOW() - INTERVAL '{interval}'
        GROUP BY COALESCE(payload->>'outcome', 'unknown')
        ORDER BY count DESC
        "#
    ))
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(outcome, count)| OutcomeBucket { outcome, count })
        .collect())
}

async fn fetch_rejection_breakdown(
    db: &PgPool,
    interval: &str,
) -> Result<Vec<RejectionBucket>, ApiError> {
    let rows: Vec<(String, String, i64)> = sqlx::query_as(&format!(
        r#"
        SELECT
          COALESCE(payload->>'tool', 'unknown') AS tool,
          COALESCE(payload->>'reason', 'unknown') AS reason,
          COUNT(*) AS count
        FROM agent_token_events
        WHERE kind = 'mcp_guard_rejection'
          AND created_at >= NOW() - INTERVAL '{interval}'
        GROUP BY tool, reason
        ORDER BY count DESC
        "#
    ))
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(tool, reason, count)| RejectionBucket {
            tool,
            reason,
            count,
        })
        .collect())
}

async fn fetch_top_repos(db: &PgPool, interval: &str) -> Result<Vec<RepoVolume>, ApiError> {
    let rows: Vec<(String, String, i64, i64, i64)> = sqlx::query_as(&format!(
        r#"
        SELECT
          repo_owner AS owner,
          repo_name AS name,
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage') AS log_usage,
          COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo') AS watch_repo,
          COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection') AS rejections
        FROM agent_token_events
        WHERE created_at >= NOW() - INTERVAL '{interval}'
          AND repo_owner IS NOT NULL
          AND repo_name IS NOT NULL
        GROUP BY repo_owner, repo_name
        ORDER BY (
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage')
          + COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo')
          + COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection')
        ) DESC
        LIMIT 20
        "#
    ))
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(owner, name, log_usage, watch_repo, rejections)| RepoVolume {
                owner,
                name,
                log_usage,
                watch_repo,
                rejections,
            },
        )
        .collect())
}

async fn fetch_top_users(db: &PgPool, interval: &str) -> Result<Vec<UserVolume>, ApiError> {
    let rows: Vec<(uuid::Uuid, i64, i64, i64)> = sqlx::query_as(&format!(
        r#"
        SELECT
          user_id,
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage') AS log_usage,
          COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo') AS watch_repo,
          COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection') AS rejections
        FROM agent_token_events
        WHERE created_at >= NOW() - INTERVAL '{interval}'
        GROUP BY user_id
        ORDER BY (
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage')
          + COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo')
          + COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection')
        ) DESC
        LIMIT 10
        "#
    ))
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(user_id, log_usage, watch_repo, rejections)| UserVolume {
            user_id: user_id.to_string(),
            log_usage,
            watch_repo,
            rejections,
        })
        .collect())
}

async fn fetch_daily_volume(db: &PgPool, interval: &str) -> Result<Vec<DailyBucket>, ApiError> {
    let rows: Vec<(chrono::DateTime<chrono::Utc>, i64, i64, i64)> = sqlx::query_as(&format!(
        r#"
        SELECT
          date_trunc('day', created_at) AS bucket,
          COUNT(*) FILTER (WHERE kind = 'mcp_log_usage') AS log_usage,
          COUNT(*) FILTER (WHERE kind = 'mcp_watch_repo') AS watch_repo,
          COUNT(*) FILTER (WHERE kind = 'mcp_guard_rejection') AS rejections
        FROM agent_token_events
        WHERE created_at >= NOW() - INTERVAL '{interval}'
        GROUP BY bucket
        ORDER BY bucket ASC
        "#
    ))
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(bucket, log_usage, watch_repo, rejections)| DailyBucket {
            bucket: bucket.to_rfc3339(),
            log_usage,
            watch_repo,
            rejections,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_window_defaults_to_week() {
        assert_eq!(MetricsWindow::parse(None).unwrap(), MetricsWindow::Week);
        assert_eq!(MetricsWindow::parse(Some("")).unwrap(), MetricsWindow::Week);
        assert_eq!(
            MetricsWindow::parse(Some("7d")).unwrap(),
            MetricsWindow::Week
        );
        assert_eq!(
            MetricsWindow::parse(Some("week")).unwrap(),
            MetricsWindow::Week
        );
    }

    #[test]
    fn parse_window_accepts_24h_and_month() {
        assert_eq!(
            MetricsWindow::parse(Some("24h")).unwrap(),
            MetricsWindow::Day
        );
        assert_eq!(
            MetricsWindow::parse(Some("1d")).unwrap(),
            MetricsWindow::Day
        );
        assert_eq!(
            MetricsWindow::parse(Some("DAY")).unwrap(),
            MetricsWindow::Day
        );
        assert_eq!(
            MetricsWindow::parse(Some("30d")).unwrap(),
            MetricsWindow::Month
        );
        assert_eq!(
            MetricsWindow::parse(Some("month")).unwrap(),
            MetricsWindow::Month
        );
    }

    #[test]
    fn parse_window_rejects_unknown() {
        let err =
            MetricsWindow::parse(Some("6h")).expect_err("'6h' should not be a recognized window");
        assert!(err.message.contains("invalid window"));
        assert!(err.message.contains("6h"));
    }

    #[test]
    fn window_maps_to_sql_interval_and_label() {
        assert_eq!(MetricsWindow::Day.as_interval(), "24 hours");
        assert_eq!(MetricsWindow::Week.as_interval(), "7 days");
        assert_eq!(MetricsWindow::Month.as_interval(), "30 days");
        assert_eq!(MetricsWindow::Day.as_str(), "24h");
        assert_eq!(MetricsWindow::Week.as_str(), "7d");
        assert_eq!(MetricsWindow::Month.as_str(), "30d");
    }
}
