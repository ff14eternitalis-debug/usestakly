use chrono::{DateTime, Utc};

use super::formula::Formula;

#[derive(Debug, Clone)]
pub struct ArtifactMetrics {
    /// Raw counts (persisted in `artifact_scores` for audit/display).
    pub resolve_count: i32,
    pub build_success_count: i32,
    pub build_failure_count: i32,
    pub regret_count: i32,
    /// Weighted counts (used by `compute_score`). Equal to raw counts
    /// only when every signal has weight 1 — e.g. pre-v1.1 legacy callers.
    pub weighted_resolve: f64,
    pub weighted_build_success: f64,
    pub weighted_build_failure: f64,
    pub weighted_regret: f64,
    pub last_update: DateTime<Utc>,
    pub flags: Vec<String>,
}

impl ArtifactMetrics {
    /// Builder for callers (tests, legacy) that don't yet distinguish
    /// raw and weighted — weighted takes the raw value.
    #[cfg(test)]
    pub fn unweighted(
        resolve_count: i32,
        build_success_count: i32,
        build_failure_count: i32,
        regret_count: i32,
        last_update: DateTime<Utc>,
        flags: Vec<String>,
    ) -> Self {
        Self {
            resolve_count,
            build_success_count,
            build_failure_count,
            regret_count,
            weighted_resolve: f64::from(resolve_count),
            weighted_build_success: f64::from(build_success_count),
            weighted_build_failure: f64::from(build_failure_count),
            weighted_regret: f64::from(regret_count),
            last_update,
            flags,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComputedScore {
    pub freshness: f64,
    pub adoption: f64,
    pub reliability: f64,
    pub abandonment: f64,
    pub overall: f64,
}

pub fn compute_score(
    metrics: &ArtifactMetrics,
    formula: &Formula,
    now: DateTime<Utc>,
) -> ComputedScore {
    let freshness = freshness_score(
        metrics.last_update,
        now,
        formula.dimensions.freshness.half_life_days,
    );
    let adoption = adoption_score(
        metrics.weighted_resolve,
        formula.dimensions.adoption.saturation,
    );
    let reliability = reliability_score(
        metrics.weighted_build_success,
        metrics.weighted_build_failure,
        f64::from(formula.dimensions.reliability.min_sample),
        formula.dimensions.reliability.neutral_default,
    );
    let regret_rate = if metrics.weighted_resolve > 0.0 {
        metrics.weighted_regret / metrics.weighted_resolve
    } else {
        0.0
    };
    let abandonment = abandonment_score(
        freshness,
        regret_rate,
        formula.dimensions.abandonment.regret_rate_threshold,
    );

    let overall = (freshness * formula.dimensions.freshness.weight
        + adoption * formula.dimensions.adoption.weight
        + reliability * formula.dimensions.reliability.weight
        + (1.0 - abandonment) * formula.dimensions.abandonment.weight)
        .clamp(0.0, 1.0);

    ComputedScore {
        freshness,
        adoption,
        reliability,
        abandonment,
        overall,
    }
}

fn freshness_score(last_update: DateTime<Utc>, now: DateTime<Utc>, half_life_days: f64) -> f64 {
    let age_days = (now - last_update).num_seconds().max(0) as f64 / 86_400.0;
    0.5_f64.powf(age_days / half_life_days).clamp(0.0, 1.0)
}

fn adoption_score(weighted_resolve: f64, saturation: f64) -> f64 {
    if weighted_resolve <= 0.0 {
        return 0.0;
    }
    let numer = (weighted_resolve + 1.0).ln();
    let denom = (saturation + 1.0).ln();
    (numer / denom).clamp(0.0, 1.0)
}

fn reliability_score(success: f64, failure: f64, min_sample: f64, neutral_default: f64) -> f64 {
    let total = success + failure;
    if total < min_sample {
        return neutral_default;
    }
    (success / total).clamp(0.0, 1.0)
}

fn abandonment_score(freshness: f64, regret_rate: f64, regret_threshold: f64) -> f64 {
    let base = 1.0 - freshness;
    let bump = if regret_rate > regret_threshold {
        (regret_rate - regret_threshold).min(0.5)
    } else {
        0.0
    };
    (base + bump).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::quality::formula::load_v1;
    use chrono::Duration;

    fn test_formula() -> Formula {
        load_v1().expect("formula v1 loads")
    }

    #[test]
    fn freshness_decays_exponentially() {
        let now = Utc::now();
        let fresh = freshness_score(now, now, 180.0);
        let six_months = freshness_score(now - Duration::days(180), now, 180.0);
        let one_year = freshness_score(now - Duration::days(360), now, 180.0);
        assert!((fresh - 1.0).abs() < 1e-9);
        assert!((six_months - 0.5).abs() < 1e-9);
        assert!((one_year - 0.25).abs() < 1e-9);
    }

    #[test]
    fn adoption_is_zero_for_no_resolves_and_grows_log() {
        assert_eq!(adoption_score(0.0, 1000.0), 0.0);
        let s10 = adoption_score(10.0, 1000.0);
        let s100 = adoption_score(100.0, 1000.0);
        let s1000 = adoption_score(1000.0, 1000.0);
        assert!(s10 < s100 && s100 < s1000);
        assert!((s1000 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn reliability_floors_on_small_sample() {
        let r = reliability_score(2.0, 0.0, 5.0, 0.5);
        assert_eq!(r, 0.5);
        let r = reliability_score(9.0, 1.0, 5.0, 0.5);
        assert!((r - 0.9).abs() < 1e-9);
    }

    #[test]
    fn overall_score_is_clamped_and_uses_weights() {
        let formula = test_formula();
        let now = Utc::now();

        let perfect = ArtifactMetrics::unweighted(1000, 100, 0, 0, now, vec![]);
        let ps = compute_score(&perfect, &formula, now);
        assert!(ps.overall > 0.85);

        let dead = ArtifactMetrics::unweighted(0, 0, 0, 0, now - Duration::days(1800), vec![]);
        let ds = compute_score(&dead, &formula, now);
        assert!(ds.overall < 0.25);
    }
}
