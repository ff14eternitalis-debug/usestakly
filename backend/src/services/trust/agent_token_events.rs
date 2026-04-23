use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, domain::quality::SignalKind, services::trust::reputation};

const EVENT_LOG_USAGE: &str = "mcp_log_usage";
const EVENT_WATCH_REPO: &str = "mcp_watch_repo";
const EVENT_GUARD_REJECTION: &str = "mcp_guard_rejection";

pub const REJECTION_REASON_QUOTA: &str = "quota_exceeded";
pub const REJECTION_REASON_DUPLICATE: &str = "duplicate_cooldown";
pub const REJECTION_REASON_NEGATIVE_WINDOW: &str = "negative_window";
pub const REJECTION_REASON_NEGATIVE_REPUTATION: &str = "negative_reputation";
pub const REJECTION_REASON_NEGATIVE_HISTORY: &str = "negative_usage_history";
pub const REJECTION_REASON_NEGATIVE_NOTES: &str = "negative_notes_too_short";

pub const REJECTION_TOOL_LOG_USAGE: &str = "log_usage";
pub const REJECTION_TOOL_WATCH_REPO: &str = "watch_repo";

pub async fn enforce_write_quota(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    tool: &str,
    owner: &str,
    name: &str,
    max_per_hour: u32,
) -> Result<(), ApiError> {
    let used: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM agent_token_events
          WHERE token_id = $1
            AND created_at >= NOW() - INTERVAL '1 hour'
            AND kind IN ('mcp_log_usage', 'mcp_watch_repo')
        "#,
    )
    .bind(token_id)
    .fetch_one(db)
    .await?;

    if used >= i64::from(max_per_hour) {
        record_rejection(
            db,
            token_id,
            user_id,
            tool,
            owner,
            name,
            REJECTION_REASON_QUOTA,
            serde_json::json!({ "limit_per_hour": max_per_hour, "used": used }),
        )
        .await
        .ok();
        return Err(ApiError::too_many_requests(format!(
            "MCP write quota exceeded for this token ({max_per_hour}/hour)"
        )));
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn enforce_log_usage_guards(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
    outcome: SignalKind,
    notes: Option<&str>,
    cooldown_secs: u64,
    negative_window_hours: u64,
) -> Result<(), ApiError> {
    let cooldown_secs =
        i32::try_from(cooldown_secs).map_err(|_| ApiError::bad_request("cooldown is too large"))?;
    let duplicate_recent: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM agent_token_events
          WHERE token_id = $1
            AND kind = $2
            AND lower(repo_owner) = lower($3)
            AND lower(repo_name) = lower($4)
            AND payload->>'outcome' = $5
            AND created_at >= NOW() - make_interval(secs => $6)
        )
        "#,
    )
    .bind(token_id)
    .bind(EVENT_LOG_USAGE)
    .bind(owner)
    .bind(name)
    .bind(outcome.as_str())
    .bind(cooldown_secs)
    .fetch_one(db)
    .await?;

    if duplicate_recent {
        record_rejection(
            db,
            token_id,
            user_id,
            REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            REJECTION_REASON_DUPLICATE,
            serde_json::json!({
                "outcome": outcome.as_str(),
                "cooldown_secs": cooldown_secs,
            }),
        )
        .await
        .ok();
        return Err(ApiError::too_many_requests(format!(
            "duplicate MCP usage signal blocked for {owner}/{name}; retry later"
        )));
    }

    if !is_negative_outcome(outcome) {
        return Ok(());
    }
    enforce_negative_outcome_reputation(db, token_id, user_id, owner, name, outcome, notes).await?;

    let negative_window_hours = i32::try_from(negative_window_hours)
        .map_err(|_| ApiError::bad_request("negative window is too large"))?;
    let recent_negative: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
          SELECT 1
          FROM agent_token_events
          WHERE user_id = $1
            AND kind = $2
            AND lower(repo_owner) = lower($3)
            AND lower(repo_name) = lower($4)
            AND payload->>'outcome' IN ('build_failure', 'regret', 're_resolve')
            AND created_at >= NOW() - make_interval(hours => $5)
        )
        "#,
    )
    .bind(user_id)
    .bind(EVENT_LOG_USAGE)
    .bind(owner)
    .bind(name)
    .bind(negative_window_hours)
    .fetch_one(db)
    .await?;

    if recent_negative {
        record_rejection(
            db,
            token_id,
            user_id,
            REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            REJECTION_REASON_NEGATIVE_WINDOW,
            serde_json::json!({
                "outcome": outcome.as_str(),
                "window_hours": negative_window_hours,
            }),
        )
        .await
        .ok();
        return Err(ApiError::too_many_requests(format!(
            "negative MCP usage signals for {owner}/{name} are limited; retry later"
        )));
    }

    Ok(())
}

pub async fn record_log_usage(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
    outcome: SignalKind,
    notes: Option<&str>,
) -> Result<(), ApiError> {
    record_event(
        db,
        token_id,
        user_id,
        EVENT_LOG_USAGE,
        owner,
        name,
        serde_json::json!({
            "outcome": outcome.as_str(),
            "notes": notes,
        }),
    )
    .await
}

pub async fn record_watch_repo(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
) -> Result<(), ApiError> {
    record_event(
        db,
        token_id,
        user_id,
        EVENT_WATCH_REPO,
        owner,
        name,
        serde_json::json!({}),
    )
    .await
}

async fn record_event(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    kind: &str,
    owner: &str,
    name: &str,
    payload: Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO agent_token_events (token_id, user_id, kind, repo_owner, repo_name, payload)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(token_id)
    .bind(user_id)
    .bind(kind)
    .bind(owner)
    .bind(name)
    .bind(payload)
    .execute(db)
    .await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn record_rejection(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    tool: &str,
    owner: &str,
    name: &str,
    reason: &str,
    extra: Value,
) -> Result<(), ApiError> {
    let mut payload = serde_json::json!({
        "tool": tool,
        "reason": reason,
    });
    if let (Some(obj), Some(extra_obj)) = (payload.as_object_mut(), extra.as_object()) {
        for (k, v) in extra_obj {
            obj.insert(k.clone(), v.clone());
        }
    }
    record_event(
        db,
        token_id,
        user_id,
        EVENT_GUARD_REJECTION,
        owner,
        name,
        payload,
    )
    .await
}

fn is_negative_outcome(outcome: SignalKind) -> bool {
    matches!(
        outcome,
        SignalKind::BuildFailure | SignalKind::Regret | SignalKind::ReResolve
    )
}

async fn enforce_negative_outcome_reputation(
    db: &PgPool,
    token_id: Uuid,
    user_id: Uuid,
    owner: &str,
    name: &str,
    outcome: SignalKind,
    notes: Option<&str>,
) -> Result<(), ApiError> {
    let rep = reputation::get_user_reputation(db, user_id).await?;
    let notes_len = notes.map(str::trim).map(str::len).unwrap_or(0);

    if rep.review_weight() < 0.55 {
        record_rejection(
            db,
            token_id,
            user_id,
            REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            REJECTION_REASON_NEGATIVE_REPUTATION,
            serde_json::json!({
                "outcome": outcome.as_str(),
                "tier": rep.tier.as_str(),
                "review_weight": rep.review_weight(),
            }),
        )
        .await
        .ok();
        return Err(ApiError::forbidden(format!(
            "negative MCP usage signals require a more established trust profile (current tier: {})",
            rep.tier.as_str()
        )));
    }

    if rep.usage_signal_count() < 5
        || rep.successful_outcome_ratio() < 0.35
        || rep.regret_ratio() > 0.45
    {
        record_rejection(
            db,
            token_id,
            user_id,
            REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            REJECTION_REASON_NEGATIVE_HISTORY,
            serde_json::json!({
                "outcome": outcome.as_str(),
                "usage_signal_count": rep.usage_signal_count(),
                "successful_outcome_ratio": rep.successful_outcome_ratio(),
                "regret_ratio": rep.regret_ratio(),
            }),
        )
        .await
        .ok();
        return Err(ApiError::forbidden(
            "negative MCP usage signals are temporarily restricted until the account builds a healthier usage history",
        ));
    }

    if matches!(outcome, SignalKind::Regret | SignalKind::ReResolve) && notes_len < 12 {
        record_rejection(
            db,
            token_id,
            user_id,
            REJECTION_TOOL_LOG_USAGE,
            owner,
            name,
            REJECTION_REASON_NEGATIVE_NOTES,
            serde_json::json!({
                "outcome": outcome.as_str(),
                "notes_len": notes_len,
            }),
        )
        .await
        .ok();
        return Err(ApiError::bad_request(
            "negative MCP usage signals need short notes (12+ chars) for review context",
        ));
    }

    Ok(())
}
