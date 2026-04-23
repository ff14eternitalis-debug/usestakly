use chrono::{DateTime, Utc};

use super::formula::Formula;

#[derive(Debug, Clone)]
pub struct ArtifactMetrics {
    pub resolve_count: i32,
    pub build_success_count: i32,
    pub build_failure_count: i32,
    pub regret_count: i32,
    pub last_update: DateTime<Utc>,
    pub flags: Vec<String>,
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
        metrics.resolve_count,
        formula.dimensions.adoption.saturation,
    );
    let reliability = reliability_score(
        metrics.build_success_count,
        metrics.build_failure_count,
        formula.dimensions.reliability.min_sample,
        formula.dimensions.reliability.neutral_default,
    );
    let regret_rate = if metrics.resolve_count > 0 {
        metrics.regret_count as f64 / metrics.resolve_count as f64
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

fn adoption_score(resolve_count: i32, saturation: f64) -> f64 {
    if resolve_count <= 0 {
        return 0.0;
    }
    let numer = ((resolve_count as f64) + 1.0).ln();
    let denom = (saturation + 1.0).ln();
    (numer / denom).clamp(0.0, 1.0)
}

fn reliability_score(success: i32, failure: i32, min_sample: u32, neutral_default: f64) -> f64 {
    let total = success.saturating_add(failure);
    if (total as u32) < min_sample {
        return neutral_default;
    }
    (success as f64 / total as f64).clamp(0.0, 1.0)
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
        assert_eq!(adoption_score(0, 1000.0), 0.0);
        let s10 = adoption_score(10, 1000.0);
        let s100 = adoption_score(100, 1000.0);
        let s1000 = adoption_score(1000, 1000.0);
        assert!(s10 < s100 && s100 < s1000);
        assert!((s1000 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn reliability_floors_on_small_sample() {
        let r = reliability_score(2, 0, 5, 0.5);
        assert_eq!(r, 0.5);
        let r = reliability_score(9, 1, 5, 0.5);
        assert!((r - 0.9).abs() < 1e-9);
    }

    #[test]
    fn overall_score_is_clamped_and_uses_weights() {
        let formula = test_formula();
        let now = Utc::now();

        let perfect = ArtifactMetrics {
            resolve_count: 1000,
            build_success_count: 100,
            build_failure_count: 0,
            regret_count: 0,
            last_update: now,
            flags: vec![],
        };
        let ps = compute_score(&perfect, &formula, now);
        assert!(ps.overall > 0.85);

        let dead = ArtifactMetrics {
            resolve_count: 0,
            build_success_count: 0,
            build_failure_count: 0,
            regret_count: 0,
            last_update: now - Duration::days(1800),
            flags: vec![],
        };
        let ds = compute_score(&dead, &formula, now);
        assert!(ds.overall < 0.25);
    }
}
