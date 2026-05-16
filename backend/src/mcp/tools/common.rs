use chrono::{DateTime, Utc};
use http::Uri;
use rmcp::ErrorData;
use rmcp::schemars;
use schemars::JsonSchema;
use serde::Serialize;

use crate::domain::reference::SearchFilter;

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Provenance {
    pub source: String,
    pub formula_version: String,
    pub scored_at: Option<DateTime<Utc>>,
}

pub(crate) fn parse_filter(s: Option<&str>) -> SearchFilter {
    match s.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        Some("strict") => SearchFilter::Strict,
        Some("explore") => SearchFilter::Explore,
        _ => SearchFilter::Auto,
    }
}

pub(crate) fn map_api_error(e: crate::app::error::ApiError) -> ErrorData {
    ErrorData::internal_error(format!("service error: {}", e.message), None)
}

pub(crate) fn map_anyhow(e: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(format!("scoring error: {e}"), None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RiskTolerance {
    Low,
    Medium,
    High,
}

impl RiskTolerance {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            RiskTolerance::Low => "low",
            RiskTolerance::Medium => "medium",
            RiskTolerance::High => "high",
        }
    }
}

pub(crate) fn parse_risk_tolerance(input: Option<&str>) -> RiskTolerance {
    match input.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        Some("low") | Some("safe") | Some("conservative") => RiskTolerance::Low,
        Some("high") | Some("experimental") | Some("explore") => RiskTolerance::High,
        _ => RiskTolerance::Medium,
    }
}

pub(crate) fn normalize_topics(topics: &[String]) -> Vec<String> {
    let mut normalized = Vec::new();
    for topic in topics {
        let topic = topic
            .trim()
            .trim_start_matches('#')
            .to_ascii_lowercase()
            .replace('_', "-");
        if !topic.is_empty() && !normalized.contains(&topic) {
            normalized.push(topic);
        }
    }
    normalized
}

pub(crate) fn push_unique(values: &mut Vec<String>, value: String) {
    let value = value.trim().trim_matches(['[', ']']).to_string();
    if value.is_empty() || values.iter().any(|existing| existing == &value) {
        return;
    }
    values.push(value);
}

pub(crate) fn mcp_allowed_hosts(config: &crate::config::AppConfig) -> Vec<String> {
    let mut hosts = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ];

    for value in [
        config.app_base_url.as_str(),
        config.frontend_base_url.as_str(),
    ] {
        if let Ok(uri) = value.parse::<Uri>()
            && let Some(authority) = uri.authority()
        {
            push_unique(&mut hosts, authority.as_str().to_string());
            push_unique(&mut hosts, authority.host().to_string());
        }
    }

    push_unique(&mut hosts, config.host.clone());
    hosts
}
