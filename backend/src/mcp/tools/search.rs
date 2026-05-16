use chrono::{DateTime, Utc};
use rmcp::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::Provenance;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchReposParams {
    /// Lexical query. Matched against owner, repo name, description, and topics.
    #[serde(default)]
    pub query: Option<String>,
    /// Quality filter preset: `auto` (default) excludes unreliable/abandoned repos,
    /// `strict` keeps only the most trusted, `explore` disables quality gates.
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub stars_min: Option<i32>,
    /// Optional comma-like list of radar maturity bands to keep:
    /// established, emerging, experimental, stale, noisy.
    #[serde(default)]
    pub maturity_bands: Vec<String>,
    /// Sort mode: score (default), stars, recency, abandonment, or trend/radar.
    #[serde(default)]
    pub sort: Option<String>,
    /// Max number of results (default 20, max 50).
    #[serde(default)]
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RepoCandidate {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub topics: Vec<String>,
    pub stars_count: i32,
    pub archived: bool,
    pub last_commit_at: Option<DateTime<Utc>>,
    pub quality_overall: Option<f64>,
    pub quality_reliability: Option<f64>,
    pub quality_abandonment: Option<f64>,
    pub flags: Vec<String>,
    pub radar: Option<RadarBrief>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct RadarBrief {
    pub maturity_band: String,
    pub radar_relevance: f64,
    pub trend_signal: f64,
    pub summary: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct SearchReposOutput {
    pub provenance: Provenance,
    pub filter_used: String,
    pub sort_used: String,
    pub count: usize,
    pub results: Vec<RepoCandidate>,
}

pub(crate) fn into_repo_candidate(repo: crate::domain::repo::RepoSearchResult) -> RepoCandidate {
    let radar = repo.radar.as_ref().map(radar_brief);
    RepoCandidate {
        owner: repo.owner,
        name: repo.name,
        full_name: repo.full_name,
        html_url: repo.html_url,
        description: repo.description,
        language: repo.language,
        license_spdx: repo.license_spdx,
        topics: repo.topics,
        stars_count: repo.stars_count,
        archived: repo.archived,
        last_commit_at: repo.last_commit_at,
        quality_overall: repo.quality.as_ref().and_then(|q| q.overall),
        quality_reliability: repo.quality.as_ref().and_then(|q| q.reliability),
        quality_abandonment: repo.quality.as_ref().and_then(|q| q.abandonment),
        flags: repo.quality.map(|q| q.flags).unwrap_or_default(),
        radar,
    }
}

pub(crate) fn radar_brief(radar: &crate::domain::repo::RepoRadarSnapshot) -> RadarBrief {
    RadarBrief {
        maturity_band: radar.maturity_band.clone(),
        radar_relevance: radar.radar_relevance,
        trend_signal: radar.trend_signal,
        summary: radar_summary(radar),
    }
}

pub(crate) fn radar_summary(radar: &crate::domain::repo::RepoRadarSnapshot) -> String {
    let reasons = radar
        .explanation
        .get("reasons")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let corpus_backed = reasons.contains(&"corpus_backed");
    let base = match radar.maturity_band.as_str() {
        "established" => "Radar: established baseline with mature quality and activity signals.",
        "emerging" => {
            "Radar: emerging repo with active signals, but usage proof is still building."
        }
        "experimental" => {
            "Radar: experimental candidate; evidence is still thin, inspect before adopting."
        }
        "stale" => "Radar: stale or flagged candidate; verify maintenance before use.",
        "noisy" => "Radar: weak category signal; treat it as a lead, not a recommendation.",
        _ => "Radar: maturity signal is available; inspect the repo context before use.",
    };
    let base = if corpus_backed {
        format!(
            "{base} Corpus-backed: strong GitHub activity; UseStakly community proof may still be pending."
        )
    } else {
        base.to_string()
    };
    if radar.trend_signal >= 0.85 {
        format!("{base} Trend signal is strong.")
    } else if radar.trend_signal >= 0.55 {
        format!("{base} Trend signal is visible.")
    } else {
        base
    }
}
