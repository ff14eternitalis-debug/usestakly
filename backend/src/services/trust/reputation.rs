use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{app::error::ApiError, domain::account::UserReputationSummary};

const PASSIVE_SATURATION: f64 = 25.0;
const RESOLVE_SATURATION: f64 = 15.0;
const ACCOUNT_AGE_SATURATION_DAYS: f64 = 90.0;

#[derive(Debug, Clone)]
pub struct UserReputation {
    pub user_id: Uuid,
    pub score: f64,
    pub account_age_days: i64,
    pub passive_signal_count: i64,
    pub resolve_count: i64,
    pub build_success_count: i64,
    pub build_failure_count: i64,
    pub regret_count: i64,
}

impl UserReputation {
    pub fn active_signal_eligible(&self, min_reputation: f64) -> bool {
        self.score >= min_reputation && self.passive_signal_count >= 5 && self.account_age_days >= 7
    }

    pub fn to_summary(&self, min_reputation: f64) -> UserReputationSummary {
        UserReputationSummary {
            user_id: self.user_id,
            score: self.score,
            account_age_days: self.account_age_days,
            passive_signal_count: self.passive_signal_count,
            resolve_count: self.resolve_count,
            build_success_count: self.build_success_count,
            build_failure_count: self.build_failure_count,
            regret_count: self.regret_count,
            active_signal_eligible: self.active_signal_eligible(min_reputation),
        }
    }
}

pub async fn get_user_reputation(db: &PgPool, user_id: Uuid) -> Result<UserReputation, ApiError> {
    let row: UserReputationRow = sqlx::query_as(
        r#"
        SELECT
          u.id AS user_id,
          u.created_at AS created_at,
          COUNT(qs.*) FILTER (WHERE qs.is_passive) AS passive_signal_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'resolve') AS resolve_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'build_success') AS build_success_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'build_failure') AS build_failure_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'regret') AS regret_count
        FROM users u
        LEFT JOIN quality_signals qs
          ON qs.actor_user_id = u.id
        WHERE u.id = $1
        GROUP BY u.id, u.created_at
        "#,
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;

    Ok(row.into_reputation())
}

pub async fn list_user_reputations(db: &PgPool) -> Result<HashMap<Uuid, UserReputation>, ApiError> {
    let rows: Vec<UserReputationRow> = sqlx::query_as(
        r#"
        SELECT
          u.id AS user_id,
          u.created_at AS created_at,
          COUNT(qs.*) FILTER (WHERE qs.is_passive) AS passive_signal_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'resolve') AS resolve_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'build_success') AS build_success_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'build_failure') AS build_failure_count,
          COUNT(qs.*) FILTER (WHERE qs.signal = 'regret') AS regret_count
        FROM users u
        LEFT JOIN quality_signals qs
          ON qs.actor_user_id = u.id
        GROUP BY u.id, u.created_at
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let reputation = row.into_reputation();
            (reputation.user_id, reputation)
        })
        .collect())
}

#[derive(FromRow)]
struct UserReputationRow {
    user_id: Uuid,
    created_at: DateTime<Utc>,
    passive_signal_count: i64,
    resolve_count: i64,
    build_success_count: i64,
    build_failure_count: i64,
    regret_count: i64,
}

impl UserReputationRow {
    fn into_reputation(self) -> UserReputation {
        let account_age_days = (Utc::now() - self.created_at).num_days().max(0);
        let age_factor = (account_age_days as f64 / ACCOUNT_AGE_SATURATION_DAYS).clamp(0.0, 1.0);
        let passive_factor =
            (self.passive_signal_count as f64 / PASSIVE_SATURATION).clamp(0.0, 1.0);
        let resolve_factor = (self.resolve_count as f64 / RESOLVE_SATURATION).clamp(0.0, 1.0);
        let build_total = self.build_success_count + self.build_failure_count;
        let reliability_factor = if build_total >= 5 {
            (self.build_success_count as f64 / build_total as f64).clamp(0.0, 1.0)
        } else {
            0.5
        };

        let score = (age_factor * 0.25
            + passive_factor * 0.35
            + resolve_factor * 0.15
            + reliability_factor * 0.25)
            .clamp(0.0, 1.0);

        UserReputation {
            user_id: self.user_id,
            score,
            account_age_days,
            passive_signal_count: self.passive_signal_count,
            resolve_count: self.resolve_count,
            build_success_count: self.build_success_count,
            build_failure_count: self.build_failure_count,
            regret_count: self.regret_count,
        }
    }
}
