use anyhow::{Context, Result};
use serde::Deserialize;

const FORMULA_V1_TOML: &str = include_str!("../../../scoring/formula_v1.toml");
const FORMULA_V2_TOML: &str = include_str!("../../../scoring/formula_v2.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct Formula {
    pub meta: FormulaMeta,
    pub dimensions: Dimensions,
    pub weighting: Weighting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormulaMeta {
    pub version: String,
    #[allow(dead_code)]
    pub created: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dimensions {
    pub freshness: FreshnessWeights,
    pub adoption: AdoptionWeights,
    pub reliability: ReliabilityWeights,
    pub abandonment: AbandonmentWeights,
    /// Optional so v1 keeps deserialising — v2 introduces this dimension.
    #[serde(default)]
    pub vitality: Option<VitalityWeights>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FreshnessWeights {
    pub weight: f64,
    pub half_life_days: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdoptionWeights {
    pub weight: f64,
    pub saturation: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReliabilityWeights {
    pub weight: f64,
    pub min_sample: u32,
    pub neutral_default: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AbandonmentWeights {
    pub weight: f64,
    pub regret_rate_threshold: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VitalityWeights {
    pub weight: f64,
    pub neutral_default: f64,
    pub contributors_saturation: f64,
    pub commits_saturation: f64,
    pub release_half_life_days: f64,
    pub collective_weight: f64,
    pub release_weight: f64,
    pub cadence_weight: f64,
    pub ci_weight: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Weighting {
    pub dedup_k: f64,
    pub outcome: OutcomeWeights,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OutcomeWeights {
    pub resolve: f64,
    pub re_resolve: f64,
    pub build_success: f64,
    pub build_failure: f64,
    pub regret: f64,
}

pub fn load_v1() -> Result<Formula> {
    toml::from_str(FORMULA_V1_TOML).context("parsing scoring/formula_v1.toml")
}

pub fn load_v2() -> Result<Formula> {
    toml::from_str(FORMULA_V2_TOML).context("parsing scoring/formula_v2.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formula_v1_loads() {
        let f = load_v1().expect("formula v1 loads");
        assert_eq!(f.meta.version, "v1.1");
        let total: f64 = f.dimensions.freshness.weight
            + f.dimensions.adoption.weight
            + f.dimensions.reliability.weight
            + f.dimensions.abandonment.weight;
        assert!((total - 1.0).abs() < 1e-9, "dimension weights sum to 1");
        assert!(
            f.dimensions.vitality.is_none(),
            "v1 must not declare vitality"
        );
    }

    #[test]
    fn formula_v1_weighting_section_loads() {
        let f = load_v1().expect("formula v1 loads");
        assert!(f.weighting.dedup_k > 0.0);
        assert!(f.weighting.outcome.regret > f.weighting.outcome.resolve);
        assert!(f.weighting.outcome.re_resolve > f.weighting.outcome.resolve);
    }

    #[test]
    fn formula_v2_loads() {
        let f = load_v2().expect("formula v2 loads");
        assert_eq!(f.meta.version, "v2.0");
        let v = f
            .dimensions
            .vitality
            .as_ref()
            .expect("v2 declares vitality");
        let total: f64 = f.dimensions.freshness.weight
            + f.dimensions.adoption.weight
            + f.dimensions.reliability.weight
            + f.dimensions.abandonment.weight
            + v.weight;
        assert!(
            (total - 1.0).abs() < 1e-9,
            "v2 dimension weights sum to 1, got {}",
            total
        );
        let sub_total = v.collective_weight + v.release_weight + v.cadence_weight + v.ci_weight;
        assert!(
            (sub_total - 1.0).abs() < 1e-9,
            "vitality sub-weights sum to 1, got {}",
            sub_total
        );
    }
}
