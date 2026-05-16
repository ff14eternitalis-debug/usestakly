use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

use crate::domain::{
    reference::{QualityContext, SearchFilter},
    repo::{
        RepoCategory, RepoProfile, RepoRadarSnapshot, RepoSearchResult, RepoSignal, ScoreSnapshot,
        VitalityInputs,
    },
};
use crate::services::repo_explain::{self, ExplainContext, ExplainRepoInput};

use super::normalize::normalize_public_signal;

#[derive(FromRow)]
pub(crate) struct RepoRow {
    pub(crate) artifact_id: Uuid,
    owner: Option<String>,
    name: Option<String>,
    html_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    license_spdx: Option<String>,
    topics: Vec<String>,
    stars_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    archived: bool,
    last_commit_at: Option<DateTime<Utc>>,
    categories: Json<Vec<RepoCategory>>,
    radar_maturity_band: Option<String>,
    radar_relevance: Option<f64>,
    radar_trend_signal: Option<f64>,
    radar_explanation: Option<Value>,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_vitality: Option<f64>,
    quality_overall: Option<f64>,
    quality_resolve_count: Option<i32>,
    quality_build_success_count: Option<i32>,
    quality_build_failure_count: Option<i32>,
    quality_regret_count: Option<i32>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    pub(crate) lexical_score: Option<f64>,
    #[allow(dead_code)]
    semantic_score: Option<f64>,
}

impl RepoRow {
    fn quality(&self) -> Option<QualityContext> {
        let formula_version = self.quality_formula_version.clone()?;
        let computed_at = self.quality_computed_at?;
        Some(QualityContext {
            formula_version,
            freshness: self.quality_freshness,
            adoption: self.quality_adoption,
            reliability: self.quality_reliability,
            abandonment: self.quality_abandonment,
            vitality: self.quality_vitality,
            overall: self.quality_overall,
            resolve_count: self.quality_resolve_count.unwrap_or_default(),
            build_success_count: self.quality_build_success_count.unwrap_or_default(),
            build_failure_count: self.quality_build_failure_count.unwrap_or_default(),
            regret_count: self.quality_regret_count.unwrap_or_default(),
            flags: self.quality_flags.clone().unwrap_or_default(),
            computed_at,
        })
    }

    fn radar(&self) -> Option<RepoRadarSnapshot> {
        Some(RepoRadarSnapshot {
            maturity_band: self.radar_maturity_band.clone()?,
            radar_relevance: self.radar_relevance?,
            trend_signal: self.radar_trend_signal?,
            explanation: self.radar_explanation.clone().unwrap_or(Value::Null),
        })
    }

    pub(crate) fn into_search_result_with_explanation(
        self,
        context: &ExplainContext,
    ) -> RepoSearchResult {
        let owner = self.owner.clone().unwrap_or_default();
        let name = self.name.clone().unwrap_or_default();
        let full_name = format!("{owner}/{name}");
        let html_url = self
            .html_url
            .clone()
            .unwrap_or_else(|| format!("https://github.com/{full_name}"));
        let quality = self.quality();
        let radar = self.radar();
        let categories = self.categories.0;
        let topics = self.topics.clone();
        let recommendation_explanation = Some(repo_explain::build_recommendation_explanation(
            ExplainRepoInput {
                topics: &topics,
                categories: &categories,
                archived: self.archived,
                quality: quality.as_ref(),
                radar: radar.as_ref(),
                lexical_score: self.lexical_score,
                context,
            },
        ));
        RepoSearchResult {
            artifact_id: self.artifact_id,
            owner,
            name,
            full_name,
            html_url,
            description: self.description,
            language: self.language,
            license_spdx: self.license_spdx,
            topics,
            stars_count: self.stars_count,
            forks_count: self.forks_count,
            open_issues_count: self.open_issues_count,
            archived: self.archived,
            last_commit_at: self.last_commit_at,
            quality,
            categories,
            radar,
            recommendation_explanation,
        }
    }
}

#[derive(FromRow)]
pub(crate) struct ProfileRow {
    artifact_id: Uuid,
    owner: Option<String>,
    name: Option<String>,
    html_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    license_spdx: Option<String>,
    topics: Vec<String>,
    stars_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    archived: bool,
    last_commit_at: Option<DateTime<Utc>>,
    categories: Json<Vec<RepoCategory>>,
    radar_maturity_band: Option<String>,
    radar_relevance: Option<f64>,
    radar_trend_signal: Option<f64>,
    radar_explanation: Option<Value>,
    subscribers_count: i32,
    default_branch: Option<String>,
    priors_fetched_at: Option<DateTime<Utc>>,
    structural_signals_at: Option<DateTime<Utc>>,
    distinct_contributors_90d: Option<i32>,
    commits_30d: Option<i32>,
    has_ci: Option<bool>,
    releases_count: Option<i32>,
    last_release_at: Option<DateTime<Utc>>,
    quality_formula_version: Option<String>,
    quality_freshness: Option<f64>,
    quality_adoption: Option<f64>,
    quality_reliability: Option<f64>,
    quality_abandonment: Option<f64>,
    quality_vitality: Option<f64>,
    quality_overall: Option<f64>,
    quality_resolve_count: Option<i32>,
    quality_build_success_count: Option<i32>,
    quality_build_failure_count: Option<i32>,
    quality_regret_count: Option<i32>,
    quality_flags: Option<Vec<String>>,
    quality_computed_at: Option<DateTime<Utc>>,
}

impl ProfileRow {
    pub(crate) fn into_profile(
        self,
        recent_signals: Vec<RepoSignal>,
        previous_overall: Option<f64>,
    ) -> RepoProfile {
        let approved_flags = self.quality_flags.clone().unwrap_or_default();
        let repo = RepoRow {
            artifact_id: self.artifact_id,
            owner: self.owner,
            name: self.name,
            html_url: self.html_url,
            description: self.description,
            language: self.language,
            license_spdx: self.license_spdx,
            topics: self.topics,
            stars_count: self.stars_count,
            forks_count: self.forks_count,
            open_issues_count: self.open_issues_count,
            archived: self.archived,
            last_commit_at: self.last_commit_at,
            categories: self.categories,
            radar_maturity_band: self.radar_maturity_band,
            radar_relevance: self.radar_relevance,
            radar_trend_signal: self.radar_trend_signal,
            radar_explanation: self.radar_explanation,
            quality_formula_version: self.quality_formula_version,
            quality_freshness: self.quality_freshness,
            quality_adoption: self.quality_adoption,
            quality_reliability: self.quality_reliability,
            quality_abandonment: self.quality_abandonment,
            quality_vitality: self.quality_vitality,
            quality_overall: self.quality_overall,
            quality_resolve_count: self.quality_resolve_count,
            quality_build_success_count: self.quality_build_success_count,
            quality_build_failure_count: self.quality_build_failure_count,
            quality_regret_count: self.quality_regret_count,
            quality_flags: self.quality_flags,
            quality_computed_at: self.quality_computed_at,
            lexical_score: None,
            semantic_score: None,
        }
        .into_search_result_with_explanation(&ExplainContext {
            filter: SearchFilter::Explore,
            ..Default::default()
        });
        let score_snapshot = repo.quality.as_ref().map(|quality| ScoreSnapshot {
            formula_version: quality.formula_version.clone(),
            overall: quality.overall,
            freshness: quality.freshness,
            adoption: quality.adoption,
            reliability: quality.reliability,
            abandonment: quality.abandonment,
            vitality: quality.vitality,
            computed_at: quality.computed_at,
            previous_formula_version: if previous_overall.is_some() {
                Some("v1.1".to_string())
            } else {
                None
            },
            previous_overall,
        });
        RepoProfile {
            repo,
            subscribers_count: self.subscribers_count,
            default_branch: self.default_branch,
            priors_fetched_at: self.priors_fetched_at,
            vitality_inputs: VitalityInputs {
                structural_signals_at: self.structural_signals_at,
                distinct_contributors_90d: self.distinct_contributors_90d,
                commits_30d: self.commits_30d,
                has_ci: self.has_ci,
                releases_count: self.releases_count,
                last_release_at: self.last_release_at,
            },
            recent_signals: recent_signals
                .into_iter()
                .filter(|signal| {
                    signal.is_passive
                        || approved_flags
                            .iter()
                            .any(|flag| flag == &normalize_public_signal(&signal.signal))
                })
                .collect(),
            score_snapshot,
        }
    }
}

#[derive(FromRow)]
pub(crate) struct SignalRow {
    pub(crate) id: Uuid,
    signal: String,
    is_passive: bool,
    evidence_url: Option<String>,
    evidence_description: Option<String>,
    review_status: String,
    review_note: Option<String>,
    disputed_at: Option<DateTime<Utc>>,
    dispute_reason: Option<String>,
    created_at: DateTime<Utc>,
}

impl SignalRow {
    pub(crate) fn into_signal(
        self,
        events: Vec<crate::domain::repo::RepoSignalEvent>,
    ) -> RepoSignal {
        RepoSignal {
            id: self.id,
            signal: self.signal,
            is_passive: self.is_passive,
            evidence_url: self.evidence_url,
            evidence_description: self.evidence_description,
            review_status: self.review_status,
            review_note: self.review_note,
            disputed_at: self.disputed_at,
            dispute_reason: self.dispute_reason,
            created_at: self.created_at,
            events,
        }
    }
}
