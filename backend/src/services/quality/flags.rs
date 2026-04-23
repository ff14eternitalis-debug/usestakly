use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::AppConfig, services::trust::reputation::UserReputation};

#[derive(sqlx::FromRow)]
pub(super) struct ActiveSignalRow {
    pub(super) external_artifact_id: Option<Uuid>,
    pub(super) signal: String,
    pub(super) actor_user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct ConsensusThresholds {
    pub min_reputation: f64,
    pub default_consensus: u32,
    pub severe_consensus: u32,
}

impl ConsensusThresholds {
    pub fn from_config(config: Option<&AppConfig>) -> Self {
        Self {
            min_reputation: config
                .map(|c| c.active_signal_min_reputation)
                .unwrap_or(0.45),
            default_consensus: config
                .map(|c| c.active_signal_default_consensus)
                .unwrap_or(2),
            severe_consensus: config
                .map(|c| c.active_signal_severe_consensus)
                .unwrap_or(3),
        }
    }
}

pub(super) fn normalize_flags(signals: Vec<String>) -> Vec<String> {
    signals
        .into_iter()
        .map(|s| match s.as_str() {
            "security_issue" => "security-issue".to_string(),
            other => other.to_string(),
        })
        .collect()
}

pub(super) fn compute_consensus_flags(
    rows: Vec<ActiveSignalRow>,
    reputations: &HashMap<Uuid, UserReputation>,
    thresholds: &ConsensusThresholds,
) -> HashMap<Uuid, Vec<String>> {
    let mut per_artifact: HashMap<Uuid, HashMap<String, HashSet<Uuid>>> = HashMap::new();
    for row in rows {
        let Some(artifact_id) = row.external_artifact_id else {
            continue;
        };
        let Some(user_id) = row.actor_user_id else {
            continue;
        };
        let Some(rep) = reputations.get(&user_id) else {
            continue;
        };
        if !rep.active_signal_eligible(thresholds.min_reputation) {
            continue;
        }

        per_artifact
            .entry(artifact_id)
            .or_default()
            .entry(row.signal)
            .or_default()
            .insert(user_id);
    }

    per_artifact
        .into_iter()
        .map(|(artifact_id, by_signal)| {
            let flags = by_signal
                .into_iter()
                .filter_map(|(signal, users)| {
                    let needed = if signal == "security_issue" || signal == "broken" {
                        thresholds.severe_consensus
                    } else {
                        thresholds.default_consensus
                    };
                    (users.len() as u32 >= needed).then_some(signal)
                })
                .collect::<Vec<_>>();
            (artifact_id, normalize_flags(flags))
        })
        .collect()
}

pub(super) async fn load_active_flag_consensus(
    db: &PgPool,
    reputations: &HashMap<Uuid, UserReputation>,
    config: Option<&AppConfig>,
) -> Result<HashMap<Uuid, Vec<String>>> {
    let rows: Vec<ActiveSignalRow> = sqlx::query_as(
        r#"
        SELECT external_artifact_id, signal::text AS signal, actor_user_id
        FROM quality_signals
        WHERE external_artifact_id IS NOT NULL
          AND is_passive = FALSE
          AND review_status = 'accepted'
          AND signal IN ('broken', 'security_issue', 'deprecated')
        "#,
    )
    .fetch_all(db)
    .await
    .context("loading active signals for consensus")?;

    let thresholds = ConsensusThresholds::from_config(config);
    Ok(compute_consensus_flags(rows, reputations, &thresholds))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eligible_user(id: Uuid, score: f64) -> UserReputation {
        UserReputation {
            user_id: id,
            score,
            tier: crate::services::trust::reputation::ReputationTier::Trusted,
            account_age_days: 30,
            passive_signal_count: 10,
            resolve_count: 3,
            re_resolve_count: 0,
            build_success_count: 0,
            build_failure_count: 0,
            regret_count: 0,
        }
    }

    fn row(artifact: Uuid, signal: &str, user: Uuid) -> ActiveSignalRow {
        ActiveSignalRow {
            external_artifact_id: Some(artifact),
            signal: signal.to_string(),
            actor_user_id: Some(user),
        }
    }

    fn default_thresholds() -> ConsensusThresholds {
        ConsensusThresholds {
            min_reputation: 0.45,
            default_consensus: 2,
            severe_consensus: 3,
        }
    }

    #[test]
    fn normalize_flags_converts_security_issue_to_kebab() {
        let out = normalize_flags(vec!["security_issue".into(), "deprecated".into()]);
        assert_eq!(out, vec!["security-issue", "deprecated"]);
    }

    #[test]
    fn user_below_reputation_threshold_is_skipped() {
        let artifact = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(u1, eligible_user(u1, 0.2));
        reputations.insert(u2, eligible_user(u2, 0.8));

        let rows = vec![
            row(artifact, "deprecated", u1),
            row(artifact, "deprecated", u2),
        ];

        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert!(out.get(&artifact).is_none_or(|flags| flags.is_empty()));
    }

    #[test]
    fn deprecated_reaches_consensus_at_two_eligible_users() {
        let artifact = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(u1, eligible_user(u1, 0.8));
        reputations.insert(u2, eligible_user(u2, 0.8));

        let rows = vec![
            row(artifact, "deprecated", u1),
            row(artifact, "deprecated", u2),
        ];

        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert_eq!(
            out.get(&artifact).map(Vec::as_slice),
            Some(&["deprecated".to_string()][..])
        );
    }

    #[test]
    fn security_issue_requires_three_eligible_users() {
        let artifact = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let u3 = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(u1, eligible_user(u1, 0.8));
        reputations.insert(u2, eligible_user(u2, 0.8));

        let rows = vec![
            row(artifact, "security_issue", u1),
            row(artifact, "security_issue", u2),
        ];
        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert!(
            out.get(&artifact).is_none_or(|flags| flags.is_empty()),
            "2 users on security_issue must not reach consensus"
        );

        reputations.insert(u3, eligible_user(u3, 0.8));
        let rows = vec![
            row(artifact, "security_issue", u1),
            row(artifact, "security_issue", u2),
            row(artifact, "security_issue", u3),
        ];
        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert_eq!(
            out.get(&artifact).map(Vec::as_slice),
            Some(&["security-issue".to_string()][..]),
            "3 eligible users on security_issue exposes normalized flag"
        );
    }

    #[test]
    fn same_user_reporting_twice_is_deduplicated() {
        let artifact = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(u1, eligible_user(u1, 0.8));

        let rows = vec![
            row(artifact, "deprecated", u1),
            row(artifact, "deprecated", u1),
        ];

        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert!(out.get(&artifact).is_none_or(|flags| flags.is_empty()));
    }

    #[test]
    fn fresh_account_even_with_high_score_is_not_eligible() {
        let artifact = Uuid::new_v4();
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(
            u1,
            UserReputation {
                user_id: u1,
                score: 0.8,
                tier: crate::services::trust::reputation::ReputationTier::Trusted,
                account_age_days: 2,
                passive_signal_count: 10,
                resolve_count: 3,
                re_resolve_count: 0,
                build_success_count: 0,
                build_failure_count: 0,
                regret_count: 0,
            },
        );
        reputations.insert(u2, eligible_user(u2, 0.8));

        let rows = vec![
            row(artifact, "deprecated", u1),
            row(artifact, "deprecated", u2),
        ];

        let out = compute_consensus_flags(rows, &reputations, &default_thresholds());
        assert!(
            out.get(&artifact).is_none_or(|flags| flags.is_empty()),
            "fresh account below 7 days must not count toward consensus"
        );
    }
}
