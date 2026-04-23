use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{app::error::ApiError, domain::account::UserReputationSummary};

const PASSIVE_SATURATION: f64 = 25.0;
const USAGE_SATURATION: f64 = 30.0;
const ACCOUNT_AGE_SATURATION_DAYS: f64 = 90.0;

#[derive(Debug, Clone)]
pub struct UserReputation {
    pub user_id: Uuid,
    pub score: f64,
    pub tier: ReputationTier,
    pub account_age_days: i64,
    pub passive_signal_count: i64,
    pub resolve_count: i64,
    pub re_resolve_count: i64,
    pub build_success_count: i64,
    pub build_failure_count: i64,
    pub regret_count: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReputationTier {
    Unproven,
    Emerging,
    Trusted,
    Core,
}

impl ReputationTier {
    fn from_score(score: f64) -> Self {
        if score >= 0.85 {
            Self::Core
        } else if score >= 0.70 {
            Self::Trusted
        } else if score >= 0.50 {
            Self::Emerging
        } else {
            Self::Unproven
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unproven => "unproven",
            Self::Emerging => "emerging",
            Self::Trusted => "trusted",
            Self::Core => "core",
        }
    }
}

impl UserReputation {
    pub fn active_signal_eligible(&self, min_reputation: f64) -> bool {
        self.score >= min_reputation
            && self.passive_signal_count >= 5
            && self.account_age_days >= 7
            && self.usage_signal_count() >= 3
    }

    pub fn review_weight(&self) -> f64 {
        match self.tier {
            ReputationTier::Core => 1.0,
            ReputationTier::Trusted => 0.8,
            ReputationTier::Emerging => 0.55,
            ReputationTier::Unproven => 0.30,
        }
    }

    pub fn requires_strict_active_review(&self) -> bool {
        self.tier == ReputationTier::Unproven || self.review_weight() < 0.6
    }

    pub fn to_summary(&self, min_reputation: f64) -> UserReputationSummary {
        UserReputationSummary {
            user_id: self.user_id,
            score: self.score,
            tier: self.tier.as_str().to_string(),
            account_age_days: self.account_age_days,
            passive_signal_count: self.passive_signal_count,
            resolve_count: self.resolve_count,
            re_resolve_count: self.re_resolve_count,
            build_success_count: self.build_success_count,
            build_failure_count: self.build_failure_count,
            regret_count: self.regret_count,
            usage_signal_count: self.usage_signal_count(),
            successful_outcome_ratio: self.successful_outcome_ratio(),
            build_reliability_ratio: self.build_reliability_ratio(),
            regret_ratio: self.regret_ratio(),
            active_signal_eligible: self.active_signal_eligible(min_reputation),
        }
    }

    pub fn usage_signal_count(&self) -> i64 {
        self.resolve_count
            + self.re_resolve_count
            + self.build_success_count
            + self.build_failure_count
            + self.regret_count
    }

    pub fn positive_outcome_count(&self) -> i64 {
        self.resolve_count + self.re_resolve_count + self.build_success_count
    }

    pub fn successful_outcome_ratio(&self) -> f64 {
        let total = self.usage_signal_count();
        if total <= 0 {
            return 0.0;
        }
        (self.positive_outcome_count() as f64 / total as f64).clamp(0.0, 1.0)
    }

    pub fn build_reliability_ratio(&self) -> f64 {
        let build_total = self.build_success_count + self.build_failure_count;
        if build_total <= 0 {
            return 0.5;
        }
        (self.build_success_count as f64 / build_total as f64).clamp(0.0, 1.0)
    }

    pub fn regret_ratio(&self) -> f64 {
        let total = self.usage_signal_count();
        if total <= 0 {
            return 0.0;
        }
        (self.regret_count as f64 / total as f64).clamp(0.0, 1.0)
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
          COUNT(qs.*) FILTER (WHERE qs.signal = 're_resolve') AS re_resolve_count,
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
          COUNT(qs.*) FILTER (WHERE qs.signal = 're_resolve') AS re_resolve_count,
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
    re_resolve_count: i64,
    build_success_count: i64,
    build_failure_count: i64,
    regret_count: i64,
}

impl UserReputationRow {
    fn into_reputation(self) -> UserReputation {
        let account_age_days = (Utc::now() - self.created_at).num_days().max(0);
        let metrics = ReputationMetrics::from_row(&self, account_age_days);
        let score = compute_reputation_score(&metrics);
        let tier = ReputationTier::from_score(score);

        UserReputation {
            user_id: self.user_id,
            score,
            tier,
            account_age_days,
            passive_signal_count: self.passive_signal_count,
            resolve_count: self.resolve_count,
            re_resolve_count: self.re_resolve_count,
            build_success_count: self.build_success_count,
            build_failure_count: self.build_failure_count,
            regret_count: self.regret_count,
        }
    }
}

#[derive(Debug, Clone)]
struct ReputationMetrics {
    age_factor: f64,
    passive_factor: f64,
    usage_factor: f64,
    successful_outcome_ratio: f64,
    build_reliability_ratio: f64,
    regret_penalty: f64,
}

impl ReputationMetrics {
    fn from_row(row: &UserReputationRow, account_age_days: i64) -> Self {
        let age_factor = (account_age_days as f64 / ACCOUNT_AGE_SATURATION_DAYS).clamp(0.0, 1.0);
        let passive_factor = (row.passive_signal_count as f64 / PASSIVE_SATURATION).clamp(0.0, 1.0);
        let usage_count = row.resolve_count
            + row.re_resolve_count
            + row.build_success_count
            + row.build_failure_count
            + row.regret_count;
        let usage_factor = (usage_count as f64 / USAGE_SATURATION).clamp(0.0, 1.0);
        let positive_count = row.resolve_count + row.re_resolve_count + row.build_success_count;
        let successful_outcome_ratio = if usage_count > 0 {
            (positive_count as f64 / usage_count as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let build_total = row.build_success_count + row.build_failure_count;
        let build_reliability_ratio = if build_total > 0 {
            (row.build_success_count as f64 / build_total as f64).clamp(0.0, 1.0)
        } else {
            0.5
        };
        let regret_penalty = if usage_count > 0 {
            (row.regret_count as f64 / usage_count as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        Self {
            age_factor,
            passive_factor,
            usage_factor,
            successful_outcome_ratio,
            build_reliability_ratio,
            regret_penalty,
        }
    }
}

fn compute_reputation_score(metrics: &ReputationMetrics) -> f64 {
    let base = metrics.age_factor * 0.15
        + metrics.passive_factor * 0.20
        + metrics.usage_factor * 0.20
        + metrics.successful_outcome_ratio * 0.20
        + metrics.build_reliability_ratio * 0.15
        + (1.0 - metrics.regret_penalty) * 0.10;

    base.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metrics(
        age_factor: f64,
        passive_factor: f64,
        usage_factor: f64,
        successful_outcome_ratio: f64,
        build_reliability_ratio: f64,
        regret_penalty: f64,
    ) -> ReputationMetrics {
        ReputationMetrics {
            age_factor,
            passive_factor,
            usage_factor,
            successful_outcome_ratio,
            build_reliability_ratio,
            regret_penalty,
        }
    }

    #[test]
    fn reputation_v2_rewards_successful_real_usage() {
        let low = compute_reputation_score(&sample_metrics(0.4, 0.2, 0.2, 0.3, 0.5, 0.4));
        let high = compute_reputation_score(&sample_metrics(0.8, 0.8, 0.8, 0.9, 0.9, 0.05));
        assert!(high > low);
    }

    #[test]
    fn reputation_v2_penalizes_regret() {
        let stable = compute_reputation_score(&sample_metrics(0.7, 0.7, 0.7, 0.8, 0.8, 0.0));
        let regretful = compute_reputation_score(&sample_metrics(0.7, 0.7, 0.7, 0.8, 0.8, 0.7));
        assert!(stable > regretful);
    }

    #[test]
    fn active_signal_eligibility_requires_min_real_usage() {
        let rep = UserReputation {
            user_id: Uuid::new_v4(),
            score: 0.9,
            tier: ReputationTier::Trusted,
            account_age_days: 30,
            passive_signal_count: 8,
            resolve_count: 1,
            re_resolve_count: 0,
            build_success_count: 1,
            build_failure_count: 0,
            regret_count: 0,
        };

        assert!(!rep.active_signal_eligible(0.45));
    }
}
