use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::services::trust::reputation::UserReputation;

use super::formula::Weighting;

const UNKNOWN_REPORTER_MULTIPLIER: f64 = 0.30;

#[derive(Debug, Clone)]
pub struct SignalObservation {
    pub signal_id: Uuid,
    pub external_artifact_id: Uuid,
    pub outcome: String,
    pub actor_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct WeightedCounts {
    pub resolve: f64,
    pub build_success: f64,
    pub build_failure: f64,
    pub regret: f64,
    pub raw_resolve: i32,
    pub raw_build_success: i32,
    pub raw_build_failure: i32,
    pub raw_regret: i32,
}

#[derive(Debug, Clone)]
pub struct SignalWeightBreakdown {
    pub signal_id: Uuid,
    pub outcome: String,
    pub actor_user_id: Option<Uuid>,
    pub reporter_tier: Option<String>,
    pub reporter_score: Option<f64>,
    pub outcome_weight: f64,
    pub reputation_multiplier: f64,
    pub dedup_multiplier: f64,
    pub weight: f64,
    pub bucket: Option<&'static str>,
    pub n_prev_same_user: u32,
    pub created_at: DateTime<Utc>,
}

fn outcome_weight(weighting: &Weighting, outcome: &str) -> Option<f64> {
    match outcome {
        "resolve" => Some(weighting.outcome.resolve),
        "re_resolve" => Some(weighting.outcome.re_resolve),
        "build_success" => Some(weighting.outcome.build_success),
        "build_failure" => Some(weighting.outcome.build_failure),
        "regret" => Some(weighting.outcome.regret),
        _ => None,
    }
}

fn bucket_for(outcome: &str) -> Option<&'static str> {
    match outcome {
        "resolve" | "re_resolve" => Some("resolve"),
        "build_success" => Some("build_success"),
        "build_failure" => Some("build_failure"),
        "regret" => Some("regret"),
        _ => None,
    }
}

fn reputation_multiplier(
    reputations: &HashMap<Uuid, UserReputation>,
    actor: Option<Uuid>,
) -> (f64, Option<String>, Option<f64>) {
    match actor.and_then(|id| reputations.get(&id)) {
        Some(rep) => (
            rep.review_weight(),
            Some(rep.tier.as_str().to_string()),
            Some(rep.score),
        ),
        None => (UNKNOWN_REPORTER_MULTIPLIER, None, None),
    }
}

fn dedup_multiplier(weighting: &Weighting, n_prev: u32) -> f64 {
    1.0 / (1.0 + weighting.dedup_k * f64::from(n_prev))
}

/// Agrège des signaux bruts (triés par `created_at`) en comptes pondérés.
/// Les signaux arrivent dans l'ordre chronologique pour que la dédup soit stable.
pub fn aggregate_weighted_counts(
    signals: &[SignalObservation],
    reputations: &HashMap<Uuid, UserReputation>,
    weighting: &Weighting,
) -> WeightedCounts {
    let mut counts = WeightedCounts::default();
    let mut prev_counts: HashMap<(Uuid, Uuid, String), u32> = HashMap::new();

    for obs in signals {
        let Some(w_outcome) = outcome_weight(weighting, &obs.outcome) else {
            continue;
        };
        let Some(bucket) = bucket_for(&obs.outcome) else {
            continue;
        };

        let (w_rep, _, _) = reputation_multiplier(reputations, obs.actor_user_id);

        let key = (
            obs.external_artifact_id,
            obs.actor_user_id.unwrap_or_else(Uuid::nil),
            obs.outcome.clone(),
        );
        let n_prev = *prev_counts.get(&key).unwrap_or(&0);
        let w_dedup = dedup_multiplier(weighting, n_prev);
        prev_counts.insert(key, n_prev + 1);

        let weight = w_outcome * w_rep * w_dedup;

        match bucket {
            "resolve" => {
                counts.resolve += weight;
                counts.raw_resolve += 1;
            }
            "build_success" => {
                counts.build_success += weight;
                counts.raw_build_success += 1;
            }
            "build_failure" => {
                counts.build_failure += weight;
                counts.raw_build_failure += 1;
            }
            "regret" => {
                counts.regret += weight;
                counts.raw_regret += 1;
            }
            _ => {}
        }
    }

    counts
}

/// Recalcule la décomposition par signal pour l'endpoint admin "explain".
/// Plus coûteux que `aggregate_weighted_counts` — conserve une entrée par signal.
pub fn explain_signals(
    signals: &[SignalObservation],
    reputations: &HashMap<Uuid, UserReputation>,
    weighting: &Weighting,
) -> Vec<SignalWeightBreakdown> {
    let mut out = Vec::with_capacity(signals.len());
    let mut prev_counts: HashMap<(Uuid, Uuid, String), u32> = HashMap::new();

    for obs in signals {
        let (w_rep, tier, score) = reputation_multiplier(reputations, obs.actor_user_id);
        let w_outcome = outcome_weight(weighting, &obs.outcome).unwrap_or(0.0);
        let bucket = bucket_for(&obs.outcome);

        let key = (
            obs.external_artifact_id,
            obs.actor_user_id.unwrap_or_else(Uuid::nil),
            obs.outcome.clone(),
        );
        let n_prev = *prev_counts.get(&key).unwrap_or(&0);
        let w_dedup = dedup_multiplier(weighting, n_prev);
        prev_counts.insert(key, n_prev + 1);

        let weight = w_outcome * w_rep * w_dedup;

        out.push(SignalWeightBreakdown {
            signal_id: obs.signal_id,
            outcome: obs.outcome.clone(),
            actor_user_id: obs.actor_user_id,
            reporter_tier: tier,
            reporter_score: score,
            outcome_weight: w_outcome,
            reputation_multiplier: w_rep,
            dedup_multiplier: w_dedup,
            weight,
            bucket,
            n_prev_same_user: n_prev,
            created_at: obs.created_at,
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::quality::formula::load_v1;
    use crate::services::trust::reputation::ReputationTier;
    use chrono::Duration;

    fn rep(user_id: Uuid, tier: ReputationTier, score: f64) -> UserReputation {
        UserReputation {
            user_id,
            score,
            tier,
            account_age_days: 90,
            passive_signal_count: 20,
            resolve_count: 10,
            re_resolve_count: 2,
            build_success_count: 5,
            build_failure_count: 1,
            regret_count: 0,
        }
    }

    fn obs(
        signal_id: Uuid,
        artifact: Uuid,
        outcome: &str,
        user: Option<Uuid>,
        minute: i64,
    ) -> SignalObservation {
        SignalObservation {
            signal_id,
            external_artifact_id: artifact,
            outcome: outcome.to_string(),
            actor_user_id: user,
            created_at: Utc::now() + Duration::minutes(minute),
        }
    }

    #[test]
    fn core_user_weighs_more_than_unproven() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let core = Uuid::new_v4();
        let unproven = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(core, rep(core, ReputationTier::Core, 0.9));
        reputations.insert(unproven, rep(unproven, ReputationTier::Unproven, 0.2));

        let core_signals = vec![obs(Uuid::new_v4(), artifact, "resolve", Some(core), 0)];
        let unproven_signals = vec![obs(Uuid::new_v4(), artifact, "resolve", Some(unproven), 0)];

        let core_counts = aggregate_weighted_counts(&core_signals, &reputations, &w);
        let unproven_counts = aggregate_weighted_counts(&unproven_signals, &reputations, &w);

        assert!(core_counts.resolve > unproven_counts.resolve);
        assert!((core_counts.resolve - 1.0).abs() < 1e-9); // review_weight(core) == 1.0
        assert!((unproven_counts.resolve - 0.30).abs() < 1e-9);
    }

    #[test]
    fn dedup_caps_spam_from_same_user() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let user = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(user, rep(user, ReputationTier::Trusted, 0.75));

        let signals: Vec<_> = (0..20)
            .map(|i| obs(Uuid::new_v4(), artifact, "resolve", Some(user), i))
            .collect();

        let counts = aggregate_weighted_counts(&signals, &reputations, &w);
        assert_eq!(counts.raw_resolve, 20);
        // With k=0.25, weight(n) = 0.8 / (1 + 0.25·n) — the series grows
        // logarithmically, so 20 spams land around 6, not 20. Enough to
        // defang casual spam; not a hard cap.
        assert!(
            counts.resolve < f64::from(counts.raw_resolve) * 0.40,
            "spam should fall under 40% of raw count, got {}",
            counts.resolve
        );
        // But the marginal weight of the 20th signal is tiny (~0.14),
        // so extending to 200 signals only roughly doubles the total.
        let single = 0.8 / (1.0 + 0.25 * 19.0);
        assert!(single < 0.2);
    }

    #[test]
    fn regret_outweighs_resolve_per_signal() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let user = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(user, rep(user, ReputationTier::Core, 0.9));

        let resolve = vec![obs(Uuid::new_v4(), artifact, "resolve", Some(user), 0)];
        let regret = vec![obs(Uuid::new_v4(), artifact, "regret", Some(user), 0)];

        let c_res = aggregate_weighted_counts(&resolve, &reputations, &w);
        let c_reg = aggregate_weighted_counts(&regret, &reputations, &w);

        assert!(c_reg.regret > c_res.resolve);
    }

    #[test]
    fn re_resolve_buckets_into_resolve_with_higher_weight() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let user = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(user, rep(user, ReputationTier::Core, 0.9));

        let first = vec![obs(Uuid::new_v4(), artifact, "resolve", Some(user), 0)];
        let re = vec![obs(Uuid::new_v4(), artifact, "re_resolve", Some(user), 1)];

        let c1 = aggregate_weighted_counts(&first, &reputations, &w);
        let c2 = aggregate_weighted_counts(&re, &reputations, &w);

        assert_eq!(c1.raw_resolve, 1);
        assert_eq!(c2.raw_resolve, 1);
        assert!(c2.resolve > c1.resolve);
    }

    #[test]
    fn unknown_reporter_falls_back_to_low_multiplier() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let reputations: HashMap<Uuid, UserReputation> = HashMap::new();

        let signals = vec![obs(Uuid::new_v4(), artifact, "resolve", None, 0)];
        let counts = aggregate_weighted_counts(&signals, &reputations, &w);
        assert!((counts.resolve - UNKNOWN_REPORTER_MULTIPLIER).abs() < 1e-9);
    }

    #[test]
    fn explain_mirrors_aggregate_total() {
        let w = load_v1().unwrap().weighting;
        let artifact = Uuid::new_v4();
        let user = Uuid::new_v4();
        let mut reputations = HashMap::new();
        reputations.insert(user, rep(user, ReputationTier::Emerging, 0.55));

        let signals = vec![
            obs(Uuid::new_v4(), artifact, "resolve", Some(user), 0),
            obs(Uuid::new_v4(), artifact, "resolve", Some(user), 1),
            obs(Uuid::new_v4(), artifact, "build_success", Some(user), 2),
            obs(Uuid::new_v4(), artifact, "regret", Some(user), 3),
        ];

        let counts = aggregate_weighted_counts(&signals, &reputations, &w);
        let breakdown = explain_signals(&signals, &reputations, &w);

        let by_bucket = breakdown
            .iter()
            .fold(WeightedCounts::default(), |mut acc, b| {
                match b.bucket {
                    Some("resolve") => acc.resolve += b.weight,
                    Some("build_success") => acc.build_success += b.weight,
                    Some("build_failure") => acc.build_failure += b.weight,
                    Some("regret") => acc.regret += b.weight,
                    _ => {}
                }
                acc
            });
        assert!((by_bucket.resolve - counts.resolve).abs() < 1e-9);
        assert!((by_bucket.build_success - counts.build_success).abs() < 1e-9);
        assert!((by_bucket.regret - counts.regret).abs() < 1e-9);
    }
}
