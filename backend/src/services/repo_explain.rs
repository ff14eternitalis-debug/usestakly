//! Public-safe recommendation explanation builders (search + profile).

use crate::domain::{
    reference::QualityContext,
    reference::SearchFilter,
    repo::{RecommendationExplanation, RepoCategory, RepoRadarSnapshot},
};

#[derive(Debug, Clone, Default)]
pub struct ExplainContext {
    pub filter: SearchFilter,
    pub query: Option<String>,
    pub query_tokens: Vec<String>,
    pub topics_filter: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ExplainRepoInput<'a> {
    pub topics: &'a [String],
    pub categories: &'a [RepoCategory],
    pub archived: bool,
    pub quality: Option<&'a QualityContext>,
    pub radar: Option<&'a RepoRadarSnapshot>,
    pub lexical_score: Option<f64>,
    pub context: &'a ExplainContext,
}

pub fn build_recommendation_explanation(input: ExplainRepoInput<'_>) -> RecommendationExplanation {
    let mut included = Vec::new();
    let mut caveats = Vec::new();

    match input.context.filter {
        SearchFilter::Auto => included.push("filter_auto_pass".to_string()),
        SearchFilter::Strict => included.push("filter_strict_pass".to_string()),
        SearchFilter::Explore => included.push("filter_explore".to_string()),
    }

    if let Some(quality) = input.quality {
        push_quality_reasons(quality, &mut included, &mut caveats);
    } else {
        caveats.push("no_score_yet".to_string());
    }

    if input.archived {
        caveats.push("archived_repo".to_string());
    }

    push_match_reasons(&input, &mut included);
    push_radar_reasons(input.radar, &mut included);

    if !input.context.topics_filter.is_empty() {
        included.push("topics_filter_applied".to_string());
    }

    dedupe_strings(&mut included);
    dedupe_strings(&mut caveats);

    RecommendationExplanation {
        included_because: included,
        caveats,
    }
}

fn push_quality_reasons(
    quality: &QualityContext,
    included: &mut Vec<String>,
    caveats: &mut Vec<String>,
) {
    if let Some(overall) = quality.overall {
        if overall >= 0.75 {
            included.push("quality_overall_strong".to_string());
        } else if overall >= 0.55 {
            included.push("quality_overall_moderate".to_string());
        }
    }
    if quality.reliability.is_some_and(|v| v >= 0.7) {
        included.push("reliability_high".to_string());
    }
    if quality.abandonment.is_some_and(|v| v <= 0.25) {
        included.push("abandonment_low".to_string());
    }
    if quality.freshness.is_some_and(|v| v >= 0.75) {
        included.push("freshness_high".to_string());
    }
    for flag in &quality.flags {
        let code = flag_code(flag);
        if !caveats.iter().any(|c| c == code) {
            caveats.push(code.to_string());
        }
    }
}

fn flag_code(flag: &str) -> &'static str {
    match flag {
        "security-issue" => "flag_security",
        "broken" => "flag_broken",
        "deprecated" => "flag_deprecated",
        "unmaintained" | "abandoned" => "flag_unmaintained",
        _ => "flag_public",
    }
}

fn push_match_reasons(input: &ExplainRepoInput<'_>, included: &mut Vec<String>) {
    if input
        .context
        .query
        .as_ref()
        .is_some_and(|q| !q.trim().is_empty())
    {
        if input.lexical_score.is_some_and(|s| s >= 0.5) {
            included.push("lexical_match_strong".to_string());
        } else {
            included.push("lexical_match".to_string());
        }
    }
    if !input.context.query_tokens.is_empty()
        && input.topics.iter().any(|topic| {
            input
                .context
                .query_tokens
                .iter()
                .any(|token| topic_contains_token(topic, token))
        })
    {
        included.push("topic_match".to_string());
    }
    if !input.context.query_tokens.is_empty()
        && input.categories.iter().any(|cat| {
            input
                .context
                .query_tokens
                .iter()
                .any(|token| cat.category.to_ascii_lowercase().contains(token))
        })
    {
        included.push("category_match".to_string());
    }
}

fn topic_contains_token(topic: &str, token: &str) -> bool {
    let topic = topic.to_ascii_lowercase();
    let token = token.to_ascii_lowercase();
    topic.contains(&token) || token.contains(&topic)
}

fn push_radar_reasons(radar: Option<&RepoRadarSnapshot>, included: &mut Vec<String>) {
    let Some(radar) = radar else {
        return;
    };
    match radar.maturity_band.as_str() {
        "emerging" | "experimental" => included.push("radar_emerging".to_string()),
        "established" => included.push("radar_established".to_string()),
        "stale" | "noisy" => included.push("radar_watch_band".to_string()),
        _ => {}
    }
}

fn dedupe_strings(values: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

pub fn auto_filter_summary_code(filter: SearchFilter) -> Option<&'static str> {
    match filter {
        SearchFilter::Auto => Some("auto_hides_low_quality"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repo::RepoRadarSnapshot;
    use chrono::Utc;
    use serde_json::json;

    fn quality(overall: f64, reliability: f64, abandonment: f64) -> QualityContext {
        QualityContext {
            formula_version: "v2.0".to_string(),
            freshness: Some(0.8),
            adoption: Some(0.7),
            reliability: Some(reliability),
            abandonment: Some(abandonment),
            vitality: Some(0.6),
            overall: Some(overall),
            resolve_count: 1,
            build_success_count: 1,
            build_failure_count: 0,
            regret_count: 0,
            flags: vec![],
            computed_at: Utc::now(),
        }
    }

    #[test]
    fn auto_filter_adds_gate_code() {
        let ctx = ExplainContext {
            filter: SearchFilter::Auto,
            query: Some("react".to_string()),
            query_tokens: vec!["react".to_string()],
            topics_filter: vec![],
        };
        let q = quality(0.82, 0.8, 0.1);
        let explanation = build_recommendation_explanation(ExplainRepoInput {
            topics: &["react".to_string()],
            categories: &[],
            archived: false,
            quality: Some(&q),
            radar: None,
            lexical_score: Some(0.7),
            context: &ctx,
        });
        assert!(
            explanation
                .included_because
                .contains(&"filter_auto_pass".to_string())
        );
        assert!(
            explanation
                .included_because
                .contains(&"quality_overall_strong".to_string())
        );
    }

    #[test]
    fn flags_surface_as_caveats() {
        let ctx = ExplainContext::default();
        let mut q = quality(0.7, 0.7, 0.2);
        q.flags = vec!["deprecated".to_string()];
        let explanation = build_recommendation_explanation(ExplainRepoInput {
            topics: &[],
            categories: &[],
            archived: false,
            quality: Some(&q),
            radar: None,
            lexical_score: None,
            context: &ctx,
        });
        assert!(explanation.caveats.contains(&"flag_deprecated".to_string()));
    }

    #[test]
    fn radar_emerging_code() {
        let ctx = ExplainContext {
            filter: SearchFilter::Explore,
            ..Default::default()
        };
        let radar = RepoRadarSnapshot {
            maturity_band: "emerging".to_string(),
            radar_relevance: 0.5,
            trend_signal: 0.4,
            explanation: json!({}),
        };
        let explanation = build_recommendation_explanation(ExplainRepoInput {
            topics: &[],
            categories: &[],
            archived: false,
            quality: None,
            radar: Some(&radar),
            lexical_score: None,
            context: &ctx,
        });
        assert!(
            explanation
                .included_because
                .contains(&"radar_emerging".to_string())
        );
    }
}
