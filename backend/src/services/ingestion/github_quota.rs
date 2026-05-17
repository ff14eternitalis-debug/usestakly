//! GitHub API quota snapshots for operators (public-launch-hardening Task 3).

use std::sync::{Mutex, OnceLock};

use chrono::{DateTime, Duration, Utc};
use http::HeaderMap;
use serde::Serialize;
use sqlx::PgPool;

use crate::config::AppConfig;

const LOW_REMAINING_THRESHOLD: i64 = 100;
const HIT_WINDOW: Duration = Duration::hours(1);
const SECONDARY_HIT_WINDOW: Duration = Duration::minutes(15);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaHeaderSnapshot {
    pub observed_at: DateTime<Utc>,
    pub context: String,
    pub remaining: Option<i64>,
    pub limit: Option<i64>,
    pub used: Option<i64>,
    pub reset_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaHitRecord {
    pub observed_at: DateTime<Utc>,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaDbSignals {
    pub rate_limit_hits_1h: i64,
    pub last_rate_limit_at: Option<DateTime<Utc>>,
    pub latest_rate_limit_reset_at: Option<DateTime<Utc>>,
    pub stale_github_repos: i64,
    pub github_repo_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaSchedulerContext {
    pub enabled: bool,
    pub ingest_max_repos_per_cycle: usize,
    pub corpus_refresh_stale_secs: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaLiveProbe {
    pub observed_at: DateTime<Utc>,
    pub remaining: i64,
    pub limit: i64,
    pub reset_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubQuotaReport {
    pub github_token_configured: bool,
    pub ingestion_status: &'static str,
    pub degraded_reasons: Vec<String>,
    pub last_header_snapshot: Option<GitHubQuotaHeaderSnapshot>,
    pub last_hit: Option<GitHubQuotaHitRecord>,
    pub db: GitHubQuotaDbSignals,
    pub scheduler: GitHubQuotaSchedulerContext,
    pub live_probe: Option<GitHubQuotaLiveProbe>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicIngestionStatus {
    Ok,
    Degraded,
    Disabled,
}

impl PublicIngestionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Degraded => "degraded",
            Self::Disabled => "disabled",
        }
    }
}

static LAST_HEADER_SNAPSHOT: OnceLock<Mutex<Option<GitHubQuotaHeaderSnapshot>>> = OnceLock::new();
static LAST_HIT: OnceLock<Mutex<Option<GitHubQuotaHitRecord>>> = OnceLock::new();

fn header_snapshot_slot() -> &'static Mutex<Option<GitHubQuotaHeaderSnapshot>> {
    LAST_HEADER_SNAPSHOT.get_or_init(|| Mutex::new(None))
}

fn hit_slot() -> &'static Mutex<Option<GitHubQuotaHitRecord>> {
    LAST_HIT.get_or_init(|| Mutex::new(None))
}

fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

pub fn record_headers_snapshot(context: &str, headers: &HeaderMap) {
    let remaining = header_str(headers, "x-ratelimit-remaining");
    let limit = header_str(headers, "x-ratelimit-limit");
    if remaining.is_none() && limit.is_none() {
        return;
    }
    let reset_at = header_str(headers, "x-ratelimit-reset")
        .and_then(|value| value.parse::<i64>().ok())
        .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0));
    let snapshot = GitHubQuotaHeaderSnapshot {
        observed_at: Utc::now(),
        context: context.to_string(),
        remaining: remaining.and_then(|value| value.parse().ok()),
        limit: limit.and_then(|value| value.parse().ok()),
        used: header_str(headers, "x-ratelimit-used")
            .and_then(|value| value.parse().ok()),
        reset_at,
    };
    if let Ok(mut slot) = header_snapshot_slot().lock() {
        *slot = Some(snapshot);
    }
}

pub fn record_limit_hit(kind: &str) {
    if let Ok(mut slot) = hit_slot().lock() {
        *slot = Some(GitHubQuotaHitRecord {
            observed_at: Utc::now(),
            kind: kind.to_string(),
        });
    }
}

pub fn last_header_snapshot() -> Option<GitHubQuotaHeaderSnapshot> {
    header_snapshot_slot().lock().ok().and_then(|s| s.clone())
}

pub fn last_limit_hit() -> Option<GitHubQuotaHitRecord> {
    hit_slot().lock().ok().and_then(|s| s.clone())
}

pub async fn load_db_signals(db: &PgPool, stale_after_secs: u64) -> Result<GitHubQuotaDbSignals, sqlx::Error> {
    let row: (i64, Option<DateTime<Utc>>, Option<DateTime<Utc>>, i64, i64) = sqlx::query_as(
        r#"
        SELECT
          COUNT(*) FILTER (
            WHERE github_last_rate_limit_at IS NOT NULL
              AND github_last_rate_limit_at > NOW() - INTERVAL '1 hour'
          )::bigint,
          MAX(github_last_rate_limit_at),
          MAX(github_rate_limit_reset_at),
          COUNT(*) FILTER (
            WHERE priors_fetched_at IS NULL
               OR priors_fetched_at < NOW() - ($1::bigint * INTERVAL '1 second')
          )::bigint,
          COUNT(*)::bigint
        FROM external_artifacts
        WHERE source = 'github'
        "#,
    )
    .bind(stale_after_secs as i64)
    .fetch_one(db)
    .await?;

    Ok(GitHubQuotaDbSignals {
        rate_limit_hits_1h: row.0,
        last_rate_limit_at: row.1,
        latest_rate_limit_reset_at: row.2,
        stale_github_repos: row.3,
        github_repo_count: row.4,
    })
}

pub fn assess_ingestion(
    config: &AppConfig,
    db: &GitHubQuotaDbSignals,
    snapshot: Option<&GitHubQuotaHeaderSnapshot>,
    hit: Option<&GitHubQuotaHitRecord>,
) -> (PublicIngestionStatus, Vec<String>) {
    let mut reasons = Vec::new();

    if config.github_token.as_deref().is_none_or(str::is_empty) {
        reasons.push("GITHUB_TOKEN is not configured".to_string());
        return (PublicIngestionStatus::Disabled, reasons);
    }

    if let Some(hit) = hit {
        if hit.kind == "secondary"
            && hit.observed_at > Utc::now() - SECONDARY_HIT_WINDOW
        {
            reasons.push("Recent GitHub secondary rate limit".to_string());
        }
        if hit.kind == "primary" && hit.observed_at > Utc::now() - HIT_WINDOW {
            reasons.push("Recent GitHub primary rate limit".to_string());
        }
    }

    if db.rate_limit_hits_1h >= 3 {
        reasons.push(format!(
            "{} artifact rate-limit markers in the last hour",
            db.rate_limit_hits_1h
        ));
    }

    if let Some(snapshot) = snapshot
        && let Some(remaining) = snapshot.remaining
    {
        if remaining <= 0 {
            reasons.push("GitHub API remaining quota is 0".to_string());
        } else if remaining <= LOW_REMAINING_THRESHOLD
            && snapshot.observed_at > Utc::now() - Duration::minutes(30)
        {
            reasons.push(format!("GitHub API remaining quota is low ({remaining})"));
        }
    }

    if reasons.is_empty() {
        (PublicIngestionStatus::Ok, reasons)
    } else {
        (PublicIngestionStatus::Degraded, reasons)
    }
}

pub async fn build_report(db: &PgPool, config: &AppConfig) -> Result<GitHubQuotaReport, sqlx::Error> {
    let db_signals = load_db_signals(db, config.corpus_refresh_stale_secs).await?;
    let snapshot = last_header_snapshot();
    let hit = last_limit_hit();
    let (ingestion_status, degraded_reasons) =
        assess_ingestion(config, &db_signals, snapshot.as_ref(), hit.as_ref());
    let live_probe = if let Some(token) = config.github_token.as_deref() {
        fetch_live_rate_limit(token).await.ok()
    } else {
        None
    };

    Ok(GitHubQuotaReport {
        github_token_configured: config
            .github_token
            .as_deref()
            .is_some_and(|t| !t.is_empty()),
        ingestion_status: ingestion_status.as_str(),
        degraded_reasons,
        last_header_snapshot: snapshot,
        last_hit: hit,
        db: db_signals,
        scheduler: GitHubQuotaSchedulerContext {
            enabled: config.scheduler_enabled,
            ingest_max_repos_per_cycle: config.ingest_max_repos_per_cycle,
            corpus_refresh_stale_secs: config.corpus_refresh_stale_secs,
        },
        live_probe,
        checked_at: Utc::now(),
    })
}

pub async fn public_ingestion_status(
    db: &PgPool,
    config: &AppConfig,
) -> Result<(PublicIngestionStatus, Option<String>), sqlx::Error> {
    let db_signals = load_db_signals(db, config.corpus_refresh_stale_secs).await?;
    let (status, reasons) = assess_ingestion(
        config,
        &db_signals,
        last_header_snapshot().as_ref(),
        last_limit_hit().as_ref(),
    );
    let message = if status == PublicIngestionStatus::Degraded {
        Some("GitHub ingestion degraded".to_string())
    } else if status == PublicIngestionStatus::Disabled {
        Some("GitHub ingestion disabled".to_string())
    } else {
        None
    };
    let _ = reasons;
    Ok((status, message))
}

#[derive(serde::Deserialize)]
struct RateLimitCore {
    limit: i64,
    remaining: i64,
    reset: i64,
}

#[derive(serde::Deserialize)]
struct RateLimitResources {
    core: RateLimitCore,
}

#[derive(serde::Deserialize)]
struct RateLimitResponse {
    resources: RateLimitResources,
}

pub async fn fetch_live_rate_limit(token: &str) -> Result<GitHubQuotaLiveProbe, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/rate_limit")
        .header("User-Agent", "UseStakly")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?
        .error_for_status()?
        .json::<RateLimitResponse>()
        .await?;
    let core = response.resources.core;
    let reset_at = DateTime::<Utc>::from_timestamp(core.reset, 0).unwrap_or_else(Utc::now);
    Ok(GitHubQuotaLiveProbe {
        observed_at: Utc::now(),
        remaining: core.remaining,
        limit: core.limit,
        reset_at,
    })
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::config::AppConfig;

    fn sample_config(token: Option<&str>) -> AppConfig {
        AppConfig {
            host: "127.0.0.1".to_string(),
            port: 4000,
            database_url: "postgres://localhost/test".to_string(),
            dev_user_id: Uuid::nil(),
            dev_user_email: "dev@example.com".to_string(),
            dev_user_username: "dev".to_string(),
            dev_user_display_name: None,
            dev_user_avatar_url: None,
            app_base_url: "http://127.0.0.1:4000".to_string(),
            frontend_base_url: "http://localhost:5173".to_string(),
            app_session_secret: None,
            app_notification_secret: None,
            github_client_id: None,
            github_client_secret: None,
            discord_client_id: None,
            discord_client_secret: None,
            admin_api_token: None,
            github_token: token.map(str::to_string),
            email_smtp_host: "smtp-relay.brevo.com".to_string(),
            email_smtp_port: 587,
            email_smtp_username: None,
            email_smtp_password: None,
            email_from_address: "noreply@usestakly.com".to_string(),
            email_from_name: "UseStakly".to_string(),
            scheduler_enabled: false,
            recompute_interval_secs: 3_600,
            digest_interval_secs: 1_800,
            corpus_refresh_stale_secs: 3_600,
            ingest_max_repos_per_cycle: 40,
            scheduler_run_on_startup: false,
            mcp_auth_failure_limit_per_minute: 30,
            mcp_read_limit_per_minute: 120,
            mcp_write_limit_per_hour: 60,
            mcp_log_usage_cooldown_secs: 900,
            mcp_negative_signal_window_hours: 24,
            active_signal_min_reputation: 0.45,
            active_signal_default_consensus: 2,
            active_signal_severe_consensus: 3,
            semantic_search_enabled: false,
            structural_stale_secs: 172_800,
            repo_refresh_cooldown_secs: 900,
            repo_refresh_user_limit_per_hour: 10,
        }
    }

    #[test]
    fn assess_disabled_without_token() {
        let config = sample_config(None);
        let db = GitHubQuotaDbSignals {
            rate_limit_hits_1h: 0,
            last_rate_limit_at: None,
            latest_rate_limit_reset_at: None,
            stale_github_repos: 0,
            github_repo_count: 0,
        };
        let (status, reasons) = assess_ingestion(&config, &db, None, None);
        assert_eq!(status, PublicIngestionStatus::Disabled);
        assert!(!reasons.is_empty());
    }

    #[test]
    fn assess_degraded_on_low_remaining_snapshot() {
        let config = sample_config(Some("token"));
        let db = GitHubQuotaDbSignals {
            rate_limit_hits_1h: 0,
            last_rate_limit_at: None,
            latest_rate_limit_reset_at: None,
            stale_github_repos: 0,
            github_repo_count: 10,
        };
        let snapshot = GitHubQuotaHeaderSnapshot {
            observed_at: Utc::now(),
            context: "test".into(),
            remaining: Some(12),
            limit: Some(5000),
            used: Some(4988),
            reset_at: None,
        };
        let (status, reasons) = assess_ingestion(&config, &db, Some(&snapshot), None);
        assert_eq!(status, PublicIngestionStatus::Degraded);
        assert!(reasons.iter().any(|r| r.contains("low")));
    }
}
