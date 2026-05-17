use sqlx::PgPool;
use uuid::Uuid;

use crate::app::error::ApiError;

pub const STATUS_COMPLETED: &str = "completed";
pub const STATUS_THROTTLED: &str = "throttled";

#[derive(Debug, Clone, Copy)]
pub struct RefreshLimitConfig {
    pub user_per_hour: u32,
    pub repo_cooldown_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshThrottleReason {
    UserHourlyQuota,
    RepoCooldown,
}

impl RefreshThrottleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserHourlyQuota => "user_hourly_quota",
            Self::RepoCooldown => "repo_cooldown",
        }
    }
}

pub fn user_limit_exceeded(used: i64, limit: u32) -> bool {
    used >= i64::from(limit)
}

pub fn repo_limit_exceeded(used: i64) -> bool {
    used >= 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshLimitsOutcome {
    Allowed,
    Throttled(RefreshThrottleReason),
}

pub async fn check_refresh_limits(
    db: &PgPool,
    user_id: Uuid,
    artifact_id: Uuid,
    config: &RefreshLimitConfig,
) -> Result<RefreshLimitsOutcome, ApiError> {
    let user_used: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM repo_refresh_events
        WHERE user_id = $1
          AND status = 'completed'
          AND created_at >= NOW() - INTERVAL '1 hour'
        "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    if user_limit_exceeded(user_used, config.user_per_hour) {
        return Ok(RefreshLimitsOutcome::Throttled(
            RefreshThrottleReason::UserHourlyQuota,
        ));
    }

    let repo_used: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)::bigint
        FROM repo_refresh_events
        WHERE artifact_id = $1
          AND status = 'completed'
          AND created_at >= NOW() - make_interval(secs => $2)
        "#,
    )
    .bind(artifact_id)
    .bind(i32::try_from(config.repo_cooldown_secs).unwrap_or(i32::MAX))
    .fetch_one(db)
    .await?;

    if repo_limit_exceeded(repo_used) {
        return Ok(RefreshLimitsOutcome::Throttled(
            RefreshThrottleReason::RepoCooldown,
        ));
    }

    Ok(RefreshLimitsOutcome::Allowed)
}

pub async fn record_refresh_event(
    db: &PgPool,
    user_id: Uuid,
    artifact_id: Uuid,
    status: &str,
    reason: Option<&str>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO repo_refresh_events (user_id, artifact_id, status, reason)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(artifact_id)
    .bind(status)
    .bind(reason)
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{repo_limit_exceeded, user_limit_exceeded};

    #[test]
    fn user_hourly_limit_blocks_at_threshold() {
        assert!(!user_limit_exceeded(9, 10));
        assert!(user_limit_exceeded(10, 10));
    }

    #[test]
    fn repo_cooldown_blocks_when_recent_refresh_exists() {
        assert!(!repo_limit_exceeded(0));
        assert!(repo_limit_exceeded(1));
    }
}
