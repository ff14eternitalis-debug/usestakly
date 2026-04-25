use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use crate::app::error::ApiError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetReference {
    pub library: String,
    pub snippet: String,
    pub version: Option<String>,
}

pub fn parse_reference(input: &str) -> Result<SnippetReference, ApiError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ApiError::bad_request("Reference must not be empty"));
    }
    let body = trimmed.strip_prefix('@').unwrap_or(trimmed);
    let (library, rest) = body.split_once(':').ok_or_else(|| {
        ApiError::bad_request("Reference must contain ':' between library and snippet")
    })?;
    let (snippet, version) = match rest.split_once('@') {
        Some((name, ver)) => (name, Some(ver.to_string())),
        None => (rest, None),
    };
    if library.is_empty() || snippet.is_empty() {
        return Err(ApiError::bad_request(
            "Library and snippet slugs must be non-empty",
        ));
    }
    if let Some(v) = &version
        && v.is_empty()
    {
        return Err(ApiError::bad_request("Version must not be empty after '@'"));
    }
    Ok(SnippetReference {
        library: library.to_string(),
        snippet: snippet.to_string(),
        version,
    })
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QualityContext {
    pub formula_version: String,
    pub freshness: Option<f64>,
    pub adoption: Option<f64>,
    pub reliability: Option<f64>,
    pub abandonment: Option<f64>,
    pub overall: Option<f64>,
    pub resolve_count: i32,
    pub build_success_count: i32,
    pub build_failure_count: i32,
    pub regret_count: i32,
    pub flags: Vec<String>,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSnippet {
    pub reference: String,
    pub snippet_id: Uuid,
    pub library_slug: String,
    pub snippet_slug: String,
    pub name: String,
    pub description: Option<String>,
    pub language: String,
    pub version: String,
    pub code: String,
    pub dependencies: Value,
    pub canonical_reference: String,
    pub quality: Option<QualityContext>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchFilter {
    #[default]
    Auto,
    Strict,
    Explore,
}

impl SearchFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            SearchFilter::Auto => "auto",
            SearchFilter::Strict => "strict",
            SearchFilter::Explore => "explore",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub snippet_id: Uuid,
    pub library_slug: String,
    pub snippet_slug: String,
    pub name: String,
    pub description: Option<String>,
    pub language: String,
    pub current_version: Option<String>,
    pub canonical_reference: String,
    pub quality: Option<QualityContext>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_reference_with_at_prefix() {
        let r = parse_reference("@react-ui:data-table@2.1.0").unwrap();
        assert_eq!(r.library, "react-ui");
        assert_eq!(r.snippet, "data-table");
        assert_eq!(r.version.as_deref(), Some("2.1.0"));
    }

    #[test]
    fn parses_reference_without_prefix_or_version() {
        let r = parse_reference("react-ui:data-table").unwrap();
        assert_eq!(r.library, "react-ui");
        assert_eq!(r.snippet, "data-table");
        assert_eq!(r.version, None);
    }

    #[test]
    fn rejects_missing_colon() {
        assert!(parse_reference("@react-ui").is_err());
    }

    #[test]
    fn rejects_empty_parts() {
        assert!(parse_reference("@:data-table").is_err());
        assert!(parse_reference("@react-ui:").is_err());
        assert!(parse_reference("@react-ui:data-table@").is_err());
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse_reference("").is_err());
        assert!(parse_reference("   ").is_err());
    }
}
