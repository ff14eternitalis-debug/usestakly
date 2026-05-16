use chrono::{DateTime, Utc};
use rmcp::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::Provenance;
use super::search::radar_brief;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RepoContextParams {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RepoContextOutput {
    pub provenance: Provenance,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub open_issues_count: i32,
    pub subscribers_count: i32,
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub default_branch: Option<String>,
    pub quality_overall: Option<f64>,
    pub quality_freshness: Option<f64>,
    pub quality_adoption: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub quality_vitality: Option<f64>,
    pub vitality_inputs: VitalityInputsOutput,
    pub quality_resolve_count: i32,
    pub quality_build_success_count: i32,
    pub quality_build_failure_count: i32,
    pub quality_regret_count: i32,
    pub flags: Vec<String>,
    pub radar: Option<super::search::RadarBrief>,
    pub recent_signals: Vec<SignalSummary>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct VitalityInputsOutput {
    pub structural_signals_at: Option<DateTime<Utc>>,
    pub distinct_contributors_90d: Option<i32>,
    pub commits_30d: Option<i32>,
    pub has_ci: Option<bool>,
    pub releases_count: Option<i32>,
    pub last_release_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SignalSummary {
    pub signal: String,
    pub is_passive: bool,
    pub evidence_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub(crate) fn into_context_output(
    profile: crate::domain::repo::RepoProfile,
    formula_version: String,
) -> RepoContextOutput {
    let q = profile.repo.quality.clone();
    let radar = profile.repo.radar.as_ref().map(radar_brief);
    let scored_at = q.as_ref().map(|q| q.computed_at);
    RepoContextOutput {
        provenance: Provenance {
            source: format!(
                "usestakly://registry/github/{}/{}",
                profile.repo.owner, profile.repo.name
            ),
            formula_version,
            scored_at,
        },
        owner: profile.repo.owner,
        name: profile.repo.name,
        full_name: profile.repo.full_name,
        html_url: profile.repo.html_url,
        description: profile.repo.description,
        language: profile.repo.language,
        license_spdx: profile.repo.license_spdx,
        topics: profile.repo.topics,
        stars_count: profile.repo.stars_count,
        forks_count: profile.repo.forks_count,
        open_issues_count: profile.repo.open_issues_count,
        subscribers_count: profile.subscribers_count,
        archived: profile.repo.archived,
        last_commit_at: profile.repo.last_commit_at,
        default_branch: profile.default_branch,
        quality_overall: q.as_ref().and_then(|q| q.overall),
        quality_freshness: q.as_ref().and_then(|q| q.freshness),
        quality_adoption: q.as_ref().and_then(|q| q.adoption),
        quality_reliability: q.as_ref().and_then(|q| q.reliability),
        quality_abandonment: q.as_ref().and_then(|q| q.abandonment),
        quality_vitality: q.as_ref().and_then(|q| q.vitality),
        vitality_inputs: VitalityInputsOutput {
            structural_signals_at: profile.vitality_inputs.structural_signals_at,
            distinct_contributors_90d: profile.vitality_inputs.distinct_contributors_90d,
            commits_30d: profile.vitality_inputs.commits_30d,
            has_ci: profile.vitality_inputs.has_ci,
            releases_count: profile.vitality_inputs.releases_count,
            last_release_at: profile.vitality_inputs.last_release_at,
        },
        quality_resolve_count: q.as_ref().map(|q| q.resolve_count).unwrap_or_default(),
        quality_build_success_count: q
            .as_ref()
            .map(|q| q.build_success_count)
            .unwrap_or_default(),
        quality_build_failure_count: q
            .as_ref()
            .map(|q| q.build_failure_count)
            .unwrap_or_default(),
        quality_regret_count: q.as_ref().map(|q| q.regret_count).unwrap_or_default(),
        flags: q.map(|q| q.flags).unwrap_or_default(),
        radar,
        recent_signals: profile
            .recent_signals
            .into_iter()
            .map(|s| SignalSummary {
                signal: s.signal,
                is_passive: s.is_passive,
                evidence_url: s.evidence_url,
                created_at: s.created_at,
            })
            .collect(),
    }
}
