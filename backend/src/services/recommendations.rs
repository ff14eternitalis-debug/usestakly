use serde::Serialize;

use crate::{
    app::error::ApiError,
    config::AppConfig,
    domain::{reference::SearchFilter, repo::RepoSearchResult},
    services::repos::{RepoSearchFilters, RepoSort, search_github_repos},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IntentConfidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseIntent {
    pub label: String,
    pub confidence: IntentConfidence,
    pub categories: Vec<String>,
    pub topics: Vec<String>,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseRecommendation {
    #[serde(flatten)]
    pub repo: RepoSearchResult,
    pub match_score: f64,
    pub recommendation_score: f64,
    pub risk: String,
    pub reason: String,
    pub matched_topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseRecommendationReport {
    pub query: String,
    pub risk_tolerance: String,
    pub intent: UseCaseIntent,
    pub recommendations: Vec<UseCaseRecommendation>,
    pub fallback_candidates: Vec<String>,
}

pub async fn recommend_for_use_case(
    db: &sqlx::PgPool,
    config: &AppConfig,
    query: &str,
    risk_tolerance: &str,
    limit: i64,
) -> Result<UseCaseRecommendationReport, ApiError> {
    let intent = parse_intent(query);
    let candidate_query = build_candidate_query(query, &intent);
    let filters = RepoSearchFilters {
        query: Some(candidate_query),
        filter: SearchFilter::Explore,
        topics: Vec::new(),
        score_min: None,
        abandonment_max: None,
        include_archived: false,
        sort: RepoSort::Score,
        limit: Some(80),
        ..RepoSearchFilters::default()
    };
    let candidates = search_github_repos(db, config, &filters).await?;
    let mut recommendations: Vec<UseCaseRecommendation> = candidates
        .into_iter()
        .filter_map(|repo| build_recommendation(repo, &intent, risk_tolerance))
        .collect();

    recommendations.sort_by(|a, b| {
        b.recommendation_score
            .partial_cmp(&a.recommendation_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    recommendations.truncate(limit.clamp(1, 20) as usize);

    Ok(UseCaseRecommendationReport {
        fallback_candidates: fallback_candidates_for(&intent),
        query: query.trim().to_string(),
        risk_tolerance: normalize_risk_tolerance(risk_tolerance).to_string(),
        intent,
        recommendations,
    })
}

pub fn parse_intent(query: &str) -> UseCaseIntent {
    let normalized = normalize_text(query);
    let mut categories = Vec::new();
    let mut topics = Vec::new();
    let mut languages = Vec::new();
    let mut labels = Vec::new();

    if contains_token_any(
        &normalized,
        &["orm", "database", "postgres", "sql", "prisma", "drizzle"],
    ) {
        labels.push("ORM TypeScript");
        push_unique(&mut categories, "orm");
        push_unique(&mut categories, "database");
        push_unique_many(
            &mut topics,
            &["orm", "database", "postgresql", "sql", "typescript"],
        );
    }
    if contains_any(&normalized, &["typescript", "ts", "node", "javascript"]) {
        push_unique(&mut topics, "typescript");
        push_unique(&mut languages, "TypeScript");
    }
    if contains_token_any(
        &normalized,
        &[
            "test",
            "tests",
            "testing",
            "e2e",
            "unit",
            "integration",
            "playwright",
            "vitest",
            "jest",
        ],
    ) {
        labels.push("Testing");
        push_unique(&mut categories, "testing");
        push_unique_many(
            &mut topics,
            &["test", "testing", "testing-tools", "e2e-testing"],
        );
    }
    if contains_any(
        &normalized,
        &[
            "video",
            "videos",
            "formation",
            "course",
            "tutorial",
            "recording",
            "screen",
        ],
    ) {
        labels.push("Creation video pedagogique");
        push_unique(&mut categories, "video-creation");
        push_unique(&mut categories, "education");
        push_unique_many(
            &mut topics,
            &[
                "video",
                "recording",
                "screen-recording",
                "animation",
                "ffmpeg",
                "education",
            ],
        );
    }
    if contains_any(&normalized, &["table", "grid", "datatable"]) {
        labels.push("Data grid UI");
        push_unique(&mut categories, "data-grid");
        push_unique(&mut categories, "ui");
        push_unique_many(
            &mut topics,
            &["table", "datatable", "grid", "react", "typescript"],
        );
    }
    if contains_token_any(
        &normalized,
        &[
            "ui",
            "kit",
            "component",
            "components",
            "design",
            "shadcn",
            "chakra",
            "radix",
            "mantine",
        ],
    ) {
        labels.push("Kit UI / composants");
        push_unique(&mut categories, "ui-kit");
        push_unique(&mut categories, "ui");
        push_unique(&mut categories, "components");
        push_unique_many(
            &mut topics,
            &[
                "components",
                "ui",
                "react",
                "design-system",
                "css",
                "tailwind",
            ],
        );
    }
    if contains_any(&normalized, &["auth", "login", "oauth", "session"]) {
        labels.push("Authentification web");
        push_unique(&mut categories, "auth");
        push_unique_many(&mut topics, &["auth", "oauth", "security", "nextjs"]);
    }

    let confidence = if !categories.is_empty() && !topics.is_empty() {
        IntentConfidence::High
    } else if !topics.is_empty() {
        IntentConfidence::Medium
    } else {
        IntentConfidence::Low
    };

    if topics.is_empty() {
        for token in normalized
            .split_whitespace()
            .filter(|token| token.len() >= 3)
        {
            push_unique(&mut topics, token);
        }
    }

    UseCaseIntent {
        label: labels
            .first()
            .copied()
            .unwrap_or("Recherche OSS")
            .to_string(),
        confidence,
        categories,
        topics,
        languages,
    }
}

pub fn score_candidate(
    quality_overall: f64,
    topic_match: f64,
    lexical_match: f64,
    ecosystem_match: f64,
) -> f64 {
    clamp01(
        quality_overall * 0.45 + topic_match * 0.30 + lexical_match * 0.15 + ecosystem_match * 0.10,
    )
}

fn build_recommendation(
    repo: RepoSearchResult,
    intent: &UseCaseIntent,
    risk_tolerance: &str,
) -> Option<UseCaseRecommendation> {
    if repo.archived {
        return None;
    }
    let quality = repo.quality.as_ref();
    let quality_overall = quality.and_then(|q| q.overall).unwrap_or(0.0);
    let abandonment = quality.and_then(|q| q.abandonment).unwrap_or(0.5);
    let flags = quality.map(|q| q.flags.as_slice()).unwrap_or(&[]);
    if flags
        .iter()
        .any(|flag| flag == "security-issue" || flag == "broken")
    {
        return None;
    }

    let matched_topics = matched_topics(&repo, intent);
    if intent.categories.iter().any(|category| category == "orm") && !matches_term(&repo, "orm") {
        return None;
    }
    if requires_category_match(intent) && category_match(&repo, intent) == 0.0 {
        return None;
    }
    let topic_match = if intent.topics.is_empty() {
        0.0
    } else {
        matched_topics.len() as f64 / intent.topics.len() as f64
    };
    let lexical_match = lexical_match(&repo, intent);
    let ecosystem_match = ecosystem_match(&repo, intent);
    let category_match = category_match(&repo, intent);
    let risk = classify_risk(quality_overall, abandonment);
    if !risk_allowed_for_tolerance(risk, normalize_risk_tolerance(risk_tolerance)) {
        return None;
    }

    let mut recommendation_score =
        score_candidate(quality_overall, topic_match, lexical_match, ecosystem_match);
    if category_match > 0.0 {
        recommendation_score = clamp01(recommendation_score + category_match * 0.12);
    }

    if abandonment > 0.35 {
        recommendation_score *= 0.72;
    } else if abandonment > 0.20 {
        recommendation_score *= 0.88;
    }
    if topic_match == 0.0 && lexical_match < 0.15 {
        return None;
    }

    Some(UseCaseRecommendation {
        reason: build_reason(&repo, intent, quality_overall, abandonment, &matched_topics),
        repo,
        match_score: clamp01(topic_match * 0.7 + lexical_match * 0.3),
        recommendation_score: clamp01(recommendation_score),
        risk: risk.to_string(),
        matched_topics,
    })
}

fn classify_risk(quality_overall: f64, abandonment: f64) -> &'static str {
    if abandonment <= 0.10 && quality_overall >= 0.70 {
        "low"
    } else if abandonment <= 0.30 && quality_overall >= 0.55 {
        "medium"
    } else {
        "high"
    }
}

fn risk_allowed_for_tolerance(risk: &str, tolerance: &str) -> bool {
    match tolerance {
        "low" => risk == "low",
        "medium" => risk != "high",
        "high" => true,
        _ => risk != "high",
    }
}

fn build_candidate_query(query: &str, intent: &UseCaseIntent) -> String {
    intent
        .topics
        .iter()
        .find(|topic| topic.len() >= 3)
        .cloned()
        .unwrap_or_else(|| query.trim().to_string())
}

fn matched_topics(repo: &RepoSearchResult, intent: &UseCaseIntent) -> Vec<String> {
    intent
        .topics
        .iter()
        .filter(|topic| matches_term(repo, topic))
        .cloned()
        .collect()
}

fn lexical_match(repo: &RepoSearchResult, intent: &UseCaseIntent) -> f64 {
    let haystack = repo_haystack(repo);
    let tokens: Vec<String> = intent
        .label
        .split_whitespace()
        .chain(intent.categories.iter().map(String::as_str))
        .map(normalize_text)
        .filter(|token| token.len() >= 3)
        .collect();
    if tokens.is_empty() {
        return 0.0;
    }
    let hits = tokens
        .iter()
        .filter(|token| haystack.contains(token.as_str()))
        .count();
    hits as f64 / tokens.len() as f64
}

fn ecosystem_match(repo: &RepoSearchResult, intent: &UseCaseIntent) -> f64 {
    let language = repo.language.as_deref().unwrap_or_default();
    if intent
        .languages
        .iter()
        .any(|expected| expected.eq_ignore_ascii_case(language))
    {
        return 1.0;
    }
    0.0
}

fn category_match(repo: &RepoSearchResult, intent: &UseCaseIntent) -> f64 {
    if intent.categories.is_empty() || repo.categories.is_empty() {
        return 0.0;
    }
    let hits = intent
        .categories
        .iter()
        .filter(|expected| {
            repo.categories
                .iter()
                .any(|category| category.category.eq_ignore_ascii_case(expected))
        })
        .count();
    hits as f64 / intent.categories.len() as f64
}

fn requires_category_match(intent: &UseCaseIntent) -> bool {
    intent.categories.iter().any(|category| {
        matches!(
            category.as_str(),
            "testing" | "ui-kit" | "auth" | "data-grid" | "video-tool"
        )
    })
}

fn repo_haystack(repo: &RepoSearchResult) -> String {
    normalize_text(&format!(
        "{} {} {} {} {}",
        repo.owner,
        repo.name,
        repo.description.as_deref().unwrap_or_default(),
        repo.language.as_deref().unwrap_or_default(),
        repo.topics.join(" ")
    ))
}

fn matches_term(repo: &RepoSearchResult, term: &str) -> bool {
    let normalized_term = normalize_text(term);
    if repo
        .topics
        .iter()
        .any(|topic| normalize_text(topic) == normalized_term)
    {
        return true;
    }
    repo_text_tokens(repo)
        .iter()
        .any(|token| token == &normalized_term)
}

fn repo_text_tokens(repo: &RepoSearchResult) -> Vec<String> {
    repo_haystack(repo)
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_string)
        .filter(|token| !token.is_empty())
        .collect()
}

fn build_reason(
    repo: &RepoSearchResult,
    intent: &UseCaseIntent,
    quality_overall: f64,
    abandonment: f64,
    matched_topics: &[String],
) -> String {
    let topic_part = if matched_topics.is_empty() {
        "correspond lexicalement au besoin".to_string()
    } else {
        format!("matche {}", matched_topics.join(", "))
    };
    format!(
        "{} est propose pour \"{}\" car il {}, avec un score qualite {:.3} et un risque d'abandon {:.3}.",
        repo.full_name, intent.label, topic_part, quality_overall, abandonment
    )
}

fn fallback_candidates_for(intent: &UseCaseIntent) -> Vec<String> {
    if intent
        .categories
        .iter()
        .any(|category| category == "video-creation")
    {
        return vec![
            "remotion-dev/remotion".to_string(),
            "obsproject/obs-studio".to_string(),
            "ffmpeg/ffmpeg".to_string(),
            "manimcommunity/manim".to_string(),
            "mifi/lossless-cut".to_string(),
        ];
    }
    if intent.categories.iter().any(|category| category == "orm") {
        return vec![
            "prisma/prisma".to_string(),
            "drizzle-team/drizzle-orm".to_string(),
            "typeorm/typeorm".to_string(),
            "sequelize/sequelize".to_string(),
            "knex/knex".to_string(),
        ];
    }
    if intent
        .categories
        .iter()
        .any(|category| category == "testing")
    {
        return vec![
            "vitest-dev/vitest".to_string(),
            "microsoft/playwright".to_string(),
            "jestjs/jest".to_string(),
            "testing-library/react-testing-library".to_string(),
            "cypress-io/cypress".to_string(),
        ];
    }
    if intent
        .categories
        .iter()
        .any(|category| category == "ui-kit" || category == "ui" || category == "components")
    {
        return vec![
            "shadcn-ui/ui".to_string(),
            "mui/material-ui".to_string(),
            "chakra-ui/chakra-ui".to_string(),
            "radix-ui/primitives".to_string(),
            "tailwindlabs/headlessui".to_string(),
            "ant-design/ant-design".to_string(),
            "mantinedev/mantine".to_string(),
        ];
    }
    Vec::new()
}

fn normalize_risk_tolerance(input: &str) -> &'static str {
    match input.trim().to_ascii_lowercase().as_str() {
        "low" | "medium" | "high" => match input.trim().to_ascii_lowercase().as_str() {
            "low" => "low",
            "high" => "high",
            _ => "medium",
        },
        _ => "medium",
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn contains_token_any(haystack: &str, needles: &[&str]) -> bool {
    haystack
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|token| needles.contains(&token))
}

fn push_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|item| item == value) {
        target.push(value.to_string());
    }
}

fn push_unique_many(target: &mut Vec<String>, values: &[&str]) {
    for value in values {
        push_unique(target, value);
    }
}

fn normalize_text(input: &str) -> String {
    input
        .to_ascii_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â'], "a")
        .replace(['î', 'ï'], "i")
        .replace(['ô'], "o")
        .replace(['ù', 'û'], "u")
        .replace(['ç'], "c")
}

fn clamp01(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{reference::QualityContext, repo::RepoCategory};

    fn repo_fixture(overall: f64, abandonment: f64) -> RepoSearchResult {
        RepoSearchResult {
            artifact_id: uuid::Uuid::nil(),
            owner: "example".to_string(),
            name: "tool".to_string(),
            full_name: "example/tool".to_string(),
            html_url: "https://github.com/example/tool".to_string(),
            description: Some("A TypeScript ORM for database applications".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: None,
            topics: vec![
                "orm".to_string(),
                "typescript".to_string(),
                "database".to_string(),
            ],
            stars_count: 1,
            forks_count: 0,
            open_issues_count: 0,
            archived: false,
            last_commit_at: None,
            quality: Some(QualityContext {
                formula_version: "test".to_string(),
                freshness: Some(0.7),
                adoption: Some(0.7),
                reliability: Some(0.7),
                abandonment: Some(abandonment),
                vitality: Some(0.7),
                overall: Some(overall),
                resolve_count: 0,
                build_success_count: 0,
                build_failure_count: 0,
                regret_count: 0,
                flags: Vec::new(),
                computed_at: chrono::Utc::now(),
            }),
            categories: Vec::new(),
            radar: None,
            recommendation_explanation: None,
        }
    }

    #[test]
    fn detects_typescript_orm_intent() {
        let intent = parse_intent("Je cherche un ORM TypeScript fiable");

        assert_eq!(intent.label, "ORM TypeScript");
        assert_eq!(intent.confidence, IntentConfidence::High);
        assert!(intent.categories.contains(&"orm".to_string()));
        assert!(intent.topics.contains(&"orm".to_string()));
        assert!(intent.topics.contains(&"typescript".to_string()));
        assert!(intent.languages.contains(&"TypeScript".to_string()));
    }

    #[test]
    fn detects_education_video_intent() {
        let intent = parse_intent("outil pour faire des videos de formation");

        assert_eq!(intent.label, "Creation video pedagogique");
        assert!(intent.categories.contains(&"video-creation".to_string()));
        assert!(intent.categories.contains(&"education".to_string()));
        assert!(intent.topics.contains(&"video".to_string()));
        assert!(intent.topics.contains(&"recording".to_string()));
    }

    #[test]
    fn detects_ui_kit_intent_from_short_tool_name() {
        let intent = parse_intent("Kit UI");

        assert_eq!(intent.label, "Kit UI / composants");
        assert_eq!(intent.confidence, IntentConfidence::High);
        assert!(intent.categories.contains(&"ui".to_string()));
        assert!(intent.categories.contains(&"components".to_string()));
        assert!(intent.topics.contains(&"components".to_string()));
    }

    #[test]
    fn detects_testing_intent_from_general_tool_need() {
        let intent = parse_intent("outil de test JavaScript");

        assert_eq!(intent.label, "Testing");
        assert_eq!(intent.confidence, IntentConfidence::High);
        assert!(intent.categories.contains(&"testing".to_string()));
        assert!(intent.topics.contains(&"test".to_string()));
        assert!(intent.topics.contains(&"testing".to_string()));
    }

    #[test]
    fn recommendation_score_rewards_quality_and_topic_match() {
        let focused = score_candidate(0.74, 1.0, 0.8, 1.0);
        let vague = score_candidate(0.9, 0.0, 0.1, 0.0);

        assert!(focused > vague);
        assert!((0.0..=1.0).contains(&focused));
    }

    #[test]
    fn term_match_does_not_treat_platform_as_orm() {
        let repo = RepoSearchResult {
            artifact_id: uuid::Uuid::nil(),
            owner: "supabase".to_string(),
            name: "supabase".to_string(),
            full_name: "supabase/supabase".to_string(),
            html_url: "https://github.com/supabase/supabase".to_string(),
            description: Some("The Postgres development platform".to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: None,
            topics: vec!["database".to_string(), "postgresql".to_string()],
            stars_count: 1,
            forks_count: 0,
            open_issues_count: 0,
            archived: false,
            last_commit_at: None,
            quality: None,
            categories: Vec::new(),
            radar: None,
            recommendation_explanation: None,
        };

        assert!(!matches_term(&repo, "orm"));
        assert!(matches_term(&repo, "database"));
    }

    #[test]
    fn clear_testing_intent_rejects_unrelated_javascript_repos() {
        let intent = parse_intent("outil de test JavaScript");
        let mut repo = repo_fixture(0.82, 0.05);
        repo.owner = "remotion-dev".to_string();
        repo.name = "remotion".to_string();
        repo.full_name = "remotion-dev/remotion".to_string();
        repo.description = Some("Make videos programmatically with React".to_string());
        repo.topics = vec![
            "javascript".to_string(),
            "react".to_string(),
            "video".to_string(),
        ];
        repo.categories = vec![RepoCategory {
            category: "video-tool".to_string(),
            confidence: 0.9,
            source: "github_metadata+readme".to_string(),
            evidence: serde_json::json!({}),
        }];

        assert!(build_recommendation(repo, &intent, "high").is_none());
    }

    #[test]
    fn candidate_query_uses_primary_topic_for_detected_intent() {
        let intent = parse_intent("outil video formation");

        assert_eq!(
            build_candidate_query("outil video formation", &intent),
            "video"
        );
    }

    #[test]
    fn candidate_query_skips_short_topics_for_ui_intents() {
        let intent = parse_intent("Kit UI");

        assert_eq!(build_candidate_query("Kit UI", &intent), "components");
    }

    #[test]
    fn low_risk_tolerance_keeps_only_low_risk_recommendations() {
        let intent = parse_intent("Je cherche un ORM TypeScript fiable");
        let low = build_recommendation(repo_fixture(0.82, 0.05), &intent, "low");
        let medium = build_recommendation(repo_fixture(0.64, 0.18), &intent, "low");
        let high = build_recommendation(repo_fixture(0.48, 0.42), &intent, "low");

        assert_eq!(low.unwrap().risk, "low");
        assert!(medium.is_none());
        assert!(high.is_none());
    }

    #[test]
    fn medium_and_high_risk_tolerances_expand_candidates() {
        let intent = parse_intent("Je cherche un ORM TypeScript fiable");

        assert_eq!(
            build_recommendation(repo_fixture(0.64, 0.18), &intent, "medium")
                .unwrap()
                .risk,
            "medium"
        );
        assert!(build_recommendation(repo_fixture(0.48, 0.42), &intent, "medium").is_none());
        assert_eq!(
            build_recommendation(repo_fixture(0.48, 0.42), &intent, "high")
                .unwrap()
                .risk,
            "high"
        );
    }
}
