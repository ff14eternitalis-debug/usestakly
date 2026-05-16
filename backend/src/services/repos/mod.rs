mod normalize;
mod profile;
mod rows;
mod search;

pub use profile::{get_repo_profile, get_repo_signals};
pub use search::search_github_repos;

use sqlx::PgPool;
use uuid::Uuid;

use crate::app::error::ApiError;
use crate::domain::reference::SearchFilter;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RepoSort {
    #[default]
    Score,
    Stars,
    Recency,
    Abandonment,
    Trend,
}

impl RepoSort {
    pub fn parse(input: Option<&str>) -> Self {
        match input
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref()
        {
            Some("stars") => Self::Stars,
            Some("recency") | Some("recent") | Some("freshness") => Self::Recency,
            Some("abandonment") | Some("risk") => Self::Abandonment,
            Some("trend") | Some("radar") | Some("emerging") => Self::Trend,
            _ => Self::Score,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Score => "score",
            Self::Stars => "stars",
            Self::Recency => "recency",
            Self::Abandonment => "abandonment",
            Self::Trend => "trend",
        }
    }
}

pub async fn find_github_artifact_id(
    db: &PgPool,
    owner: &str,
    name: &str,
) -> Result<Option<Uuid>, ApiError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id
        FROM external_artifacts
        WHERE source = 'github'
          AND github_owner = $1
          AND github_repo = $2
        LIMIT 1
        "#,
    )
    .bind(owner)
    .bind(name)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|(id,)| id))
}

#[derive(Debug, Clone, Default)]
pub struct RepoSearchFilters {
    pub query: Option<String>,
    pub filter: SearchFilter,
    pub language: Option<String>,
    pub license_spdx: Option<String>,
    pub stars_min: Option<i32>,
    pub topics: Vec<String>,
    pub maturity_bands: Vec<String>,
    pub score_min: Option<f64>,
    pub abandonment_max: Option<f64>,
    pub include_archived: bool,
    pub sort: RepoSort,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::RepoSort;

    #[test]
    fn repo_sort_parses_trend_aliases() {
        assert_eq!(RepoSort::parse(Some("trend")), RepoSort::Trend);
        assert_eq!(RepoSort::parse(Some("radar")), RepoSort::Trend);
        assert_eq!(RepoSort::parse(Some("emerging")), RepoSort::Trend);
    }
}
