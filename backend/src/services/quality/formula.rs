use anyhow::{Context, Result};
use serde::Deserialize;

const FORMULA_V1_TOML: &str = include_str!("../../../scoring/formula_v1.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct Formula {
    pub meta: FormulaMeta,
    pub dimensions: Dimensions,
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

pub fn load_v1() -> Result<Formula> {
    toml::from_str(FORMULA_V1_TOML).context("parsing scoring/formula_v1.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formula_v1_loads() {
        let f = load_v1().expect("formula v1 loads");
        assert_eq!(f.meta.version, "v1");
        let total: f64 = f.dimensions.freshness.weight
            + f.dimensions.adoption.weight
            + f.dimensions.reliability.weight
            + f.dimensions.abandonment.weight;
        assert!((total - 1.0).abs() < 1e-9, "dimension weights sum to 1");
    }
}
