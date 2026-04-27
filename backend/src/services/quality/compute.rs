use chrono::{DateTime, Utc};

use super::formula::{Formula, VitalityWeights};

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
    /// Structural vitality inputs (lot 1/3, migration 0018). All optional :
    /// `structural_signals_at = None` means we never captured for this repo
    /// and the formula returns the neutral default ; per-field None means a
    /// partial fetch failure and is treated as neutral 0.5 inside the blend.
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub distinct_contributors_90d: Option<i32>,
    pub commits_30d: Option<i32>,
    pub has_ci: Option<bool>,
    pub last_release_at: Option<DateTime<Utc>>,
}

impl ArtifactMetrics {
    /// Builder for callers (tests, legacy) that don't yet distinguish
    /// raw and weighted — weighted takes the raw value, no vitality input.
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
            structural_signals_at: None,
            distinct_contributors_90d: None,
            commits_30d: None,
            has_ci: None,
            last_release_at: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComputedScore {
    pub freshness: f64,
    pub adoption: f64,
    pub reliability: f64,
    pub abandonment: f64,
    pub vitality: f64,
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
    let abandonment_time = abandonment_score(
        freshness,
        regret_rate,
        formula.dimensions.abandonment.regret_rate_threshold,
    );

    let (vitality, vitality_contribution, abandonment) = match formula.dimensions.vitality.as_ref()
    {
        Some(v) => {
            let score = vitality_score(metrics, v, now);
            // v2 : a fresh-pushed but solo / no-CI / no-release repo is
            // de-facto abandoned. Couple abandonment with structural vitality
            // so freshness alone cannot mask a degraded maintainer structure.
            let coupled = abandonment_time.max(1.0 - score);
            (score, score * v.weight, coupled)
        }
        // v1 formula : dimension absent, no coupling, no contribution to overall.
        None => (0.0, 0.0, abandonment_time),
    };

    let overall = (freshness * formula.dimensions.freshness.weight
        + adoption * formula.dimensions.adoption.weight
        + reliability * formula.dimensions.reliability.weight
        + (1.0 - abandonment) * formula.dimensions.abandonment.weight
        + vitality_contribution)
        .clamp(0.0, 1.0);

    ComputedScore {
        freshness,
        adoption,
        reliability,
        abandonment,
        vitality,
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

pub fn vitality_score(
    metrics: &ArtifactMetrics,
    weights: &VitalityWeights,
    now: DateTime<Utc>,
) -> f64 {
    if metrics.structural_signals_at.is_none() {
        return weights.neutral_default;
    }

    let neutral = weights.neutral_default;
    let collective = match metrics.distinct_contributors_90d {
        Some(c) => saturate(f64::from(c) / weights.contributors_saturation),
        None => neutral,
    };
    let cadence = match metrics.commits_30d {
        Some(c) => saturate(f64::from(c) / weights.commits_saturation),
        None => neutral,
    };
    let ci = match metrics.has_ci {
        Some(true) => 1.0,
        Some(false) => 0.0,
        None => neutral,
    };
    let release = match metrics.last_release_at {
        Some(t) => freshness_score(t, now, weights.release_half_life_days),
        None => neutral,
    };

    (weights.collective_weight * collective
        + weights.release_weight * release
        + weights.cadence_weight * cadence
        + weights.ci_weight * ci)
        .clamp(0.0, 1.0)
}

fn saturate(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::quality::formula::{load_v1, load_v2};
    use chrono::Duration;

    fn v1_formula() -> Formula {
        load_v1().expect("formula v1 loads")
    }

    fn v2_formula() -> Formula {
        load_v2().expect("formula v2 loads")
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
        let formula = v1_formula();
        let now = Utc::now();

        let perfect = ArtifactMetrics::unweighted(1000, 100, 0, 0, now, vec![]);
        let ps = compute_score(&perfect, &formula, now);
        assert!(ps.overall > 0.85);

        let dead = ArtifactMetrics::unweighted(0, 0, 0, 0, now - Duration::days(1800), vec![]);
        let ds = compute_score(&dead, &formula, now);
        assert!(ds.overall < 0.25);
    }

    #[test]
    fn vitality_returns_neutral_when_never_captured() {
        let formula = v2_formula();
        let weights = formula
            .dimensions
            .vitality
            .as_ref()
            .expect("v2 has vitality");
        let now = Utc::now();
        let metrics = ArtifactMetrics::unweighted(0, 0, 0, 0, now, vec![]);
        let v = vitality_score(&metrics, weights, now);
        assert!((v - weights.neutral_default).abs() < 1e-9);
    }

    #[test]
    fn vitality_per_field_null_treated_as_neutral() {
        let formula = v2_formula();
        let weights = formula
            .dimensions
            .vitality
            .as_ref()
            .expect("v2 has vitality");
        let now = Utc::now();
        // Captured but every sub-signal missing → still neutral (0.5).
        let mut metrics = ArtifactMetrics::unweighted(0, 0, 0, 0, now, vec![]);
        metrics.structural_signals_at = Some(now);
        let v = vitality_score(&metrics, weights, now);
        assert!(
            (v - weights.neutral_default).abs() < 1e-9,
            "all-NULL captured = neutral, got {}",
            v
        );
    }

    #[test]
    fn vitality_max_for_collective_repo() {
        let formula = v2_formula();
        let weights = formula
            .dimensions
            .vitality
            .as_ref()
            .expect("v2 has vitality");
        let now = Utc::now();
        let mut metrics = ArtifactMetrics::unweighted(0, 0, 0, 0, now, vec![]);
        metrics.structural_signals_at = Some(now);
        metrics.distinct_contributors_90d = Some(20);
        metrics.commits_30d = Some(50);
        metrics.has_ci = Some(true);
        metrics.last_release_at = Some(now);
        let v = vitality_score(&metrics, weights, now);
        assert!(
            (v - 1.0).abs() < 1e-9,
            "saturated collective repo = 1.0, got {}",
            v
        );
    }

    #[test]
    fn vitality_min_for_solo_no_ci_no_release_repo() {
        let formula = v2_formula();
        let weights = formula
            .dimensions
            .vitality
            .as_ref()
            .expect("v2 has vitality");
        let now = Utc::now();
        let mut metrics = ArtifactMetrics::unweighted(0, 0, 0, 0, now, vec![]);
        metrics.structural_signals_at = Some(now);
        metrics.distinct_contributors_90d = Some(1);
        metrics.commits_30d = Some(2);
        metrics.has_ci = Some(false);
        metrics.last_release_at = None; // never released → neutral 0.5
        let v = vitality_score(&metrics, weights, now);
        // collective ≈ 0.20, cadence ≈ 0.20, ci = 0, release = 0.5 (neutral)
        // Weighted: 0.30*0.20 + 0.20*0.20 + 0.20*0 + 0.30*0.5 = 0.06 + 0.04 + 0 + 0.15 = 0.25
        assert!(
            v < 0.30,
            "solo vibe-coded repo should score low on vitality, got {}",
            v
        );
    }

    #[test]
    fn v2_overall_blocks_solo_fresh_slop_from_auto_threshold() {
        // Critère de succès du plan : un repo solo sans CI, sans release,
        // freshly-pushed, ne peut PAS atteindre `auto` (overall ≥ 0.45)
        // sur la seule fraîcheur.
        let formula = v2_formula();
        let now = Utc::now();
        let mut slop = ArtifactMetrics::unweighted(0, 0, 0, 0, now, vec![]);
        slop.structural_signals_at = Some(now);
        slop.distinct_contributors_90d = Some(1);
        slop.commits_30d = Some(3);
        slop.has_ci = Some(false);
        slop.last_release_at = None;
        let s = compute_score(&slop, &formula, now);
        assert!(
            s.overall < 0.45,
            "fresh solo no-CI no-release repo must stay below auto threshold, got overall={}",
            s.overall
        );
    }

    #[test]
    fn v2_overall_lets_collective_established_repo_pass_strict() {
        let formula = v2_formula();
        let now = Utc::now();
        let mut healthy = ArtifactMetrics::unweighted(500, 50, 0, 0, now, vec![]);
        healthy.structural_signals_at = Some(now);
        healthy.distinct_contributors_90d = Some(15);
        healthy.commits_30d = Some(40);
        healthy.has_ci = Some(true);
        healthy.last_release_at = Some(now - Duration::days(30));
        let s = compute_score(&healthy, &formula, now);
        assert!(
            s.overall > 0.70,
            "established collective repo should clear strict, got overall={}",
            s.overall
        );
    }
}
