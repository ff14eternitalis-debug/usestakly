use chrono::{DateTime, Utc};

use crate::{
    domain::{
        quality_display::{DimensionState, ProofTier},
        reference::QualityContext,
        repo::VitalityInputs,
    },
    services::quality::compute::ComputedScore,
};

pub struct DimensionStateInput<'a> {
    pub score: &'a ComputedScore,
    pub quality: Option<&'a QualityContext>,
    pub vitality: &'a VitalityInputs,
    pub reliability_min_sample: f64,
    pub now: DateTime<Utc>,
}

pub fn build_dimension_states(input: &DimensionStateInput<'_>) -> Vec<DimensionState> {
    vec![
        freshness_state(input),
        adoption_state(input),
        reliability_state(input),
        abandonment_state(input),
        vitality_state(input),
    ]
}

pub fn derive_proof_tier(states: &[DimensionState]) -> ProofTier {
    let adoption = states.iter().find(|s| s.key == "adoption");
    let reliability = states.iter().find(|s| s.key == "reliability");
    let adoption_measured = adoption.is_some_and(|s| s.display_state == "measured");
    let reliability_measured = reliability.is_some_and(|s| s.display_state == "measured");
    if adoption_measured && reliability_measured {
        return ProofTier::CommunityBacked;
    }
    let adoption_awaiting = adoption.is_some_and(|s| s.display_state == "awaiting_community");
    let usage_limited = states
        .iter()
        .any(|s| s.key == "adoption" && s.display_state == "growing")
        || (reliability.is_some_and(|s| s.display_state == "neutral_default")
            && !adoption_awaiting);
    if usage_limited {
        return ProofTier::UsageLimited;
    }
    ProofTier::CorpusOnly
}

pub fn build_dimension_states_from_quality(
    quality: Option<&QualityContext>,
    vitality: &VitalityInputs,
    reliability_min_sample: f64,
    now: DateTime<Utc>,
) -> Vec<DimensionState> {
    let score = ComputedScore {
        freshness: quality.and_then(|q| q.freshness).unwrap_or(0.0),
        adoption: quality.and_then(|q| q.adoption).unwrap_or(0.0),
        reliability: quality.and_then(|q| q.reliability).unwrap_or(0.0),
        abandonment: quality.and_then(|q| q.abandonment).unwrap_or(0.0),
        vitality: quality.and_then(|q| q.vitality).unwrap_or(0.0),
        overall: quality.and_then(|q| q.overall).unwrap_or(0.0),
    };
    build_dimension_states(&DimensionStateInput {
        score: &score,
        quality,
        vitality,
        reliability_min_sample,
        now,
    })
}

pub fn proof_tier_str(tier: ProofTier) -> &'static str {
    match tier {
        ProofTier::CorpusOnly => "corpus_only",
        ProofTier::UsageLimited => "usage_limited",
        ProofTier::CommunityBacked => "community_backed",
    }
}

fn freshness_state(input: &DimensionStateInput<'_>) -> DimensionState {
    let as_of = input
        .vitality
        .structural_signals_at
        .or_else(|| input.quality.map(|q| q.computed_at))
        .unwrap_or(input.now);
    if input.score.freshness > 0.0 || input.vitality.structural_signals_at.is_some() {
        return DimensionState {
            key: "freshness".to_string(),
            value: Some(input.score.freshness),
            display_state: "measured".to_string(),
            source: "github_metadata".to_string(),
            confidence: "high".to_string(),
            as_of,
            summary: "Derived from the latest default-branch commit timestamp.".to_string(),
        };
    }
    DimensionState {
        key: "freshness".to_string(),
        value: None,
        display_state: "missing_commit".to_string(),
        source: "github_metadata".to_string(),
        confidence: "low".to_string(),
        as_of: input.now,
        summary: "No commit timestamp is available yet.".to_string(),
    }
}

fn adoption_state(input: &DimensionStateInput<'_>) -> DimensionState {
    let resolve_count = input.quality.map(|q| q.resolve_count).unwrap_or(0);
    let as_of = input.quality.map(|q| q.computed_at).unwrap_or(input.now);
    if resolve_count == 0 {
        return DimensionState {
            key: "adoption".to_string(),
            value: Some(0.0),
            display_state: "awaiting_community".to_string(),
            source: "usage_signals".to_string(),
            confidence: "high".to_string(),
            as_of,
            summary: "No weighted UseStakly resolve signals yet; adoption stays at zero."
                .to_string(),
        };
    }
    let display_state = if resolve_count >= 10 {
        "measured"
    } else {
        "growing"
    };
    DimensionState {
        key: "adoption".to_string(),
        value: Some(input.score.adoption),
        display_state: display_state.to_string(),
        source: "usage_signals".to_string(),
        confidence: if resolve_count >= 5 { "high" } else { "medium" }.to_string(),
        as_of,
        summary: format!(
            "Based on {resolve_count} recorded resolve signal(s) from UseStakly usage."
        ),
    }
}

fn reliability_state(input: &DimensionStateInput<'_>) -> DimensionState {
    let builds = input
        .quality
        .map(|q| q.build_success_count + q.build_failure_count)
        .unwrap_or(0);
    let as_of = input.quality.map(|q| q.computed_at).unwrap_or(input.now);
    if f64::from(builds) < input.reliability_min_sample {
        return DimensionState {
            key: "reliability".to_string(),
            value: Some(input.score.reliability),
            display_state: "neutral_default".to_string(),
            source: "usage_signals".to_string(),
            confidence: "medium".to_string(),
            as_of,
            summary: format!(
                "Fewer than {} build outcomes recorded; reliability uses the neutral default.",
                input.reliability_min_sample as i32
            ),
        };
    }
    DimensionState {
        key: "reliability".to_string(),
        value: Some(input.score.reliability),
        display_state: "measured".to_string(),
        source: "usage_signals".to_string(),
        confidence: "high".to_string(),
        as_of,
        summary: "Computed from weighted build success and failure signals.".to_string(),
    }
}

fn abandonment_state(input: &DimensionStateInput<'_>) -> DimensionState {
    let as_of = input.quality.map(|q| q.computed_at).unwrap_or(input.now);
    DimensionState {
        key: "abandonment".to_string(),
        value: Some(input.score.abandonment),
        display_state: "measured".to_string(),
        source: "formula_derived".to_string(),
        confidence: "high".to_string(),
        as_of,
        summary: "Combines freshness decay with weighted regret signals when present.".to_string(),
    }
}

fn vitality_state(input: &DimensionStateInput<'_>) -> DimensionState {
    let captured = input.vitality.structural_signals_at;
    if captured.is_none() {
        return DimensionState {
            key: "vitality".to_string(),
            value: input.quality.and_then(|q| q.vitality),
            display_state: "not_captured".to_string(),
            source: "github_metadata".to_string(),
            confidence: "low".to_string(),
            as_of: input.now,
            summary: "Structural GitHub signals have not been captured for this repo yet."
                .to_string(),
        };
    }
    let mut missing = Vec::new();
    if input.vitality.distinct_contributors_90d.is_none() {
        missing.push("contributors");
    }
    if input.vitality.commits_30d.is_none() {
        missing.push("commits");
    }
    if input.vitality.has_ci.is_none() {
        missing.push("ci");
    }
    if input.vitality.releases_count.is_none() && input.vitality.last_release_at.is_none() {
        missing.push("releases");
    }
    let as_of = captured.unwrap_or(input.now);
    if !missing.is_empty() {
        return DimensionState {
            key: "vitality".to_string(),
            value: Some(input.score.vitality),
            display_state: "partial".to_string(),
            source: "github_metadata".to_string(),
            confidence: "medium".to_string(),
            as_of,
            summary: format!(
                "Some structural fields are missing ({}).",
                missing.join(", ")
            ),
        };
    }
    DimensionState {
        key: "vitality".to_string(),
        value: Some(input.score.vitality),
        display_state: "measured".to_string(),
        source: "github_metadata".to_string(),
        confidence: "high".to_string(),
        as_of,
        summary: "Based on contributors, commit cadence, CI, and release activity.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::reference::QualityContext;

    fn ffmpeg_like_quality() -> QualityContext {
        QualityContext {
            formula_version: "v2.0".to_string(),
            freshness: Some(0.99),
            adoption: Some(0.0),
            reliability: Some(0.5),
            abandonment: Some(0.01),
            vitality: Some(0.65),
            overall: Some(0.59),
            resolve_count: 0,
            build_success_count: 0,
            build_failure_count: 0,
            regret_count: 0,
            flags: vec![],
            computed_at: Utc::now(),
        }
    }

    #[test]
    fn ffmpeg_like_adoption_awaits_community() {
        let quality = ffmpeg_like_quality();
        let score = ComputedScore {
            freshness: 0.99,
            adoption: 0.0,
            reliability: 0.5,
            abandonment: 0.01,
            vitality: 0.65,
            overall: 0.59,
        };
        let vitality = VitalityInputs {
            structural_signals_at: Some(Utc::now()),
            distinct_contributors_90d: Some(64),
            commits_30d: Some(246),
            has_ci: Some(true),
            releases_count: Some(120),
            last_release_at: Some(Utc::now()),
            ..Default::default()
        };
        let states = build_dimension_states(&DimensionStateInput {
            score: &score,
            quality: Some(&quality),
            vitality: &vitality,
            reliability_min_sample: 5.0,
            now: Utc::now(),
        });
        let adoption = states.iter().find(|s| s.key == "adoption").unwrap();
        assert_eq!(adoption.display_state, "awaiting_community");
        assert_eq!(derive_proof_tier(&states), ProofTier::CorpusOnly);
    }
}
