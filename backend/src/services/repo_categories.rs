use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app::error::ApiError, domain::repo::RepoCategory,
    services::ingestion::github::GitHubRepoMetadata,
};

const SOURCE: &str = "github_metadata";
const README_SOURCE: &str = "github_metadata+readme";

#[derive(Debug, Clone, Copy)]
struct CategoryRule {
    category: &'static str,
    strong: &'static [&'static str],
    medium: &'static [&'static str],
    weak: &'static [&'static str],
}

const RULES: &[CategoryRule] = &[
    CategoryRule {
        category: "ui-kit",
        strong: &[
            "component",
            "components",
            "design-system",
            "design system",
            "shadcn",
            "material-ui",
            "chakra",
            "radix",
            "mantine",
            "headlessui",
            "ant-design",
        ],
        medium: &["ui", "react", "tailwind", "css", "frontend"],
        weak: &["kit", "interface"],
    },
    CategoryRule {
        category: "orm",
        strong: &["orm", "prisma", "drizzle", "typeorm", "sequelize", "gorm"],
        medium: &["database", "postgres", "postgresql", "sql"],
        weak: &["query-builder", "schema"],
    },
    CategoryRule {
        category: "auth",
        strong: &["auth", "authentication", "oauth", "openid", "session"],
        medium: &["login", "security", "identity"],
        weak: &["jwt", "password"],
    },
    CategoryRule {
        category: "data-grid",
        strong: &["datatable", "data-grid", "data grid", "table", "grid"],
        medium: &["spreadsheet", "react-table", "tanstack"],
        weak: &["rows", "columns"],
    },
    CategoryRule {
        category: "video-tool",
        strong: &[
            "video",
            "screen-recording",
            "recording",
            "ffmpeg",
            "remotion",
        ],
        medium: &["animation", "course", "tutorial", "education"],
        weak: &["media", "capture"],
    },
    CategoryRule {
        category: "testing",
        strong: &["test", "testing", "playwright", "vitest", "jest"],
        medium: &["e2e", "assertion", "browser"],
        weak: &["spec", "mock"],
    },
    CategoryRule {
        category: "http-client",
        strong: &["http-client", "http client", "fetch", "axios", "ky", "got"],
        medium: &["http", "request", "api"],
        weak: &["client", "network"],
    },
    CategoryRule {
        category: "validation",
        strong: &["validation", "validator", "schema", "zod", "valibot", "yup"],
        medium: &["json-schema", "types"],
        weak: &["parse", "safeparse"],
    },
];

pub fn classify_repo(meta: &GitHubRepoMetadata) -> Vec<RepoCategory> {
    classify_repo_with_readme(meta, None)
}

pub fn classify_repo_with_readme(
    meta: &GitHubRepoMetadata,
    readme: Option<&str>,
) -> Vec<RepoCategory> {
    let haystack = normalized_haystack(meta);
    let readme_haystack = readme.map(normalize_text).unwrap_or_default();
    let topic_text = meta
        .topics
        .iter()
        .map(|topic| normalize_text(topic))
        .collect::<Vec<_>>();

    let mut categories = RULES
        .iter()
        .filter_map(|rule| classify_with_rule(rule, &haystack, &readme_haystack, &topic_text))
        .collect::<Vec<_>>();

    categories.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.category.cmp(&b.category))
    });
    categories
}

pub async fn upsert_repo_categories(
    db: &PgPool,
    artifact_id: Uuid,
    meta: &GitHubRepoMetadata,
) -> Result<Vec<RepoCategory>, ApiError> {
    upsert_repo_categories_with_readme(db, artifact_id, meta, None).await
}

pub async fn upsert_repo_categories_with_readme(
    db: &PgPool,
    artifact_id: Uuid,
    meta: &GitHubRepoMetadata,
    readme: Option<&str>,
) -> Result<Vec<RepoCategory>, ApiError> {
    let categories = classify_repo_with_readme(meta, readme);
    let mut tx = db.begin().await?;

    sqlx::query("DELETE FROM repo_categories WHERE external_artifact_id = $1")
        .bind(artifact_id)
        .execute(&mut *tx)
        .await?;

    for category in &categories {
        sqlx::query(
            r#"
            INSERT INTO repo_categories (
              external_artifact_id, category, confidence, source, evidence, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, NOW())
            "#,
        )
        .bind(artifact_id)
        .bind(&category.category)
        .bind(category.confidence)
        .bind(&category.source)
        .bind(&category.evidence)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(categories)
}

pub async fn backfill_missing_repo_categories(db: &PgPool) -> Result<u64, ApiError> {
    let rows = sqlx::query_as::<_, BackfillRepoRow>(
        r#"
        SELECT
          id,
          github_id,
          github_owner,
          github_repo,
          default_branch,
          html_url,
          description,
          language,
          license_spdx,
          topics,
          archived,
          stars_count,
          forks_count,
          open_issues_count,
          subscribers_count,
          last_commit_at,
          distinct_contributors_90d,
          commits_30d,
          has_ci,
          releases_count,
          last_release_at,
          structural_signals_at
        FROM external_artifacts e
        WHERE source = 'github'
          AND github_id IS NOT NULL
          AND github_owner IS NOT NULL
          AND github_repo IS NOT NULL
          AND NOT EXISTS (
            SELECT 1
            FROM repo_categories rc
            WHERE rc.external_artifact_id = e.id
          )
        "#,
    )
    .fetch_all(db)
    .await?;

    let mut changed = 0_u64;
    for row in rows {
        let artifact_id = row.id;
        let meta = row.into_metadata();
        let categories = upsert_repo_categories(db, artifact_id, &meta).await?;
        if !categories.is_empty() {
            changed = changed.saturating_add(1);
        }
    }

    Ok(changed)
}

#[derive(sqlx::FromRow)]
struct BackfillRepoRow {
    id: Uuid,
    github_id: Option<i64>,
    github_owner: Option<String>,
    github_repo: Option<String>,
    default_branch: Option<String>,
    html_url: Option<String>,
    description: Option<String>,
    language: Option<String>,
    license_spdx: Option<String>,
    topics: Vec<String>,
    archived: bool,
    stars_count: i32,
    forks_count: i32,
    open_issues_count: i32,
    subscribers_count: i32,
    last_commit_at: Option<chrono::DateTime<chrono::Utc>>,
    distinct_contributors_90d: Option<i32>,
    commits_30d: Option<i32>,
    has_ci: Option<bool>,
    releases_count: Option<i32>,
    last_release_at: Option<chrono::DateTime<chrono::Utc>>,
    structural_signals_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl BackfillRepoRow {
    fn into_metadata(self) -> GitHubRepoMetadata {
        GitHubRepoMetadata {
            github_id: self.github_id.unwrap_or_default(),
            owner: self.github_owner.unwrap_or_default(),
            name: self.github_repo.unwrap_or_default(),
            default_branch: self.default_branch,
            html_url: self.html_url.unwrap_or_default(),
            description: self.description,
            language: self.language,
            license_spdx: self.license_spdx,
            topics: self.topics,
            archived: self.archived,
            stars_count: self.stars_count,
            forks_count: self.forks_count,
            open_issues_count: self.open_issues_count,
            subscribers_count: self.subscribers_count,
            last_commit_at: self.last_commit_at,
            structural: crate::services::ingestion::github::StructuralSignals {
                distinct_contributors_90d: self.distinct_contributors_90d,
                commits_30d: self.commits_30d,
                has_ci: self.has_ci,
                releases_count: self.releases_count,
                last_release_at: self.last_release_at,
                captured_at: self.structural_signals_at,
            },
        }
    }
}

fn classify_with_rule(
    rule: &CategoryRule,
    haystack: &str,
    readme_haystack: &str,
    topics: &[String],
) -> Option<RepoCategory> {
    let strong = matches(rule.strong, haystack, topics);
    let medium = matches(rule.medium, haystack, topics);
    let weak = matches(rule.weak, haystack, topics);
    let readme_strong = matches_text(rule.strong, readme_haystack);
    let readme_medium = matches_text(rule.medium, readme_haystack);
    let readme_weak = matches_text(rule.weak, readme_haystack);

    if rule.category == "ui-kit" {
        if strong.is_empty() && readme_strong.is_empty() {
            return None;
        }
        if strong.is_empty() && !has_ui_kit_readme_anchor(&readme_strong) {
            return None;
        }
    }
    if rule.category == "testing"
        && strong.is_empty()
        && !has_testing_readme_anchor(&readme_strong, &readme_medium)
    {
        return None;
    }
    if rule.category == "data-grid"
        && strong.is_empty()
        && !has_data_grid_readme_anchor(&readme_strong, &readme_medium)
    {
        return None;
    }

    let score = (strong.len() as f64 * 0.34)
        + (medium.len() as f64 * 0.16)
        + (weak.len() as f64 * 0.08)
        + (readme_strong.len() as f64 * 0.24)
        + (readme_medium.len() as f64 * 0.12)
        + (readme_weak.len() as f64 * 0.04);
    if score < 0.24 {
        return None;
    }

    let mut evidence_terms = Vec::new();
    evidence_terms.extend(strong.iter().cloned());
    evidence_terms.extend(medium.iter().cloned());
    evidence_terms.extend(weak.iter().cloned());
    evidence_terms.extend(readme_strong.iter().cloned());
    evidence_terms.extend(readme_medium.iter().cloned());
    evidence_terms.extend(readme_weak.iter().cloned());
    evidence_terms.sort_unstable();
    evidence_terms.dedup();

    let mut readme_terms = Vec::new();
    readme_terms.extend(readme_strong.iter().cloned());
    readme_terms.extend(readme_medium.iter().cloned());
    readme_terms.extend(readme_weak.iter().cloned());
    readme_terms.sort_unstable();
    readme_terms.dedup();

    Some(RepoCategory {
        category: rule.category.to_string(),
        confidence: score.clamp(0.0, 0.98),
        source: if readme_terms.is_empty() {
            SOURCE
        } else {
            README_SOURCE
        }
        .to_string(),
        evidence: json!({
            "matched": evidence_terms,
            "strong": strong,
            "medium": medium,
            "weak": weak,
            "readmeMatched": readme_terms
        }),
    })
}

fn matches<'a>(terms: &'a [&'a str], haystack: &str, topics: &[String]) -> Vec<&'a str> {
    let tokens = text_tokens(haystack);
    terms
        .iter()
        .copied()
        .filter(|term| {
            let normalized = normalize_text(term);
            topics.iter().any(|topic| topic == &normalized)
                || if normalized.contains('-') || normalized.contains(' ') {
                    haystack.contains(&normalized)
                } else {
                    tokens.iter().any(|token| token == &normalized)
                }
        })
        .collect()
}

fn matches_text<'a>(terms: &'a [&'a str], haystack: &str) -> Vec<&'a str> {
    if haystack.is_empty() {
        return Vec::new();
    }
    let tokens = text_tokens(haystack);
    terms
        .iter()
        .copied()
        .filter(|term| {
            let normalized = normalize_text(term);
            if normalized.contains('-') || normalized.contains(' ') {
                haystack.contains(&normalized)
            } else {
                tokens.iter().any(|token| token == &normalized)
            }
        })
        .collect()
}

fn has_ui_kit_readme_anchor(terms: &[&str]) -> bool {
    terms.iter().any(|term| {
        matches!(
            *term,
            "design-system"
                | "design system"
                | "shadcn"
                | "material-ui"
                | "chakra"
                | "radix"
                | "mantine"
                | "headlessui"
                | "ant-design"
        )
    })
}

fn has_testing_readme_anchor(strong: &[&str], medium: &[&str]) -> bool {
    strong
        .iter()
        .chain(medium.iter())
        .any(|term| matches!(*term, "testing" | "playwright" | "vitest" | "jest" | "e2e"))
}

fn has_data_grid_readme_anchor(strong: &[&str], medium: &[&str]) -> bool {
    strong.iter().chain(medium.iter()).any(|term| {
        matches!(
            *term,
            "datatable" | "data-grid" | "data grid" | "react-table" | "tanstack"
        )
    })
}

fn text_tokens(input: &str) -> Vec<String> {
    input
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_string)
        .filter(|token| !token.is_empty())
        .collect()
}

fn normalized_haystack(meta: &GitHubRepoMetadata) -> String {
    normalize_text(&format!(
        "{} {} {} {} {}",
        meta.owner,
        meta.name,
        meta.description.as_deref().unwrap_or_default(),
        meta.language.as_deref().unwrap_or_default(),
        meta.topics.join(" ")
    ))
}

fn normalize_text(input: &str) -> String {
    input
        .to_ascii_lowercase()
        .replace(['_', '/'], "-")
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â'], "a")
        .replace(['î', 'ï'], "i")
        .replace(['ô'], "o")
        .replace(['ù', 'û'], "u")
        .replace(['ç'], "c")
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::services::ingestion::github::StructuralSignals;

    fn meta(owner: &str, name: &str, description: &str, topics: &[&str]) -> GitHubRepoMetadata {
        GitHubRepoMetadata {
            github_id: 1,
            owner: owner.to_string(),
            name: name.to_string(),
            default_branch: Some("main".to_string()),
            html_url: format!("https://github.com/{owner}/{name}"),
            description: Some(description.to_string()),
            language: Some("TypeScript".to_string()),
            license_spdx: None,
            topics: topics.iter().map(|topic| topic.to_string()).collect(),
            archived: false,
            stars_count: 1,
            forks_count: 0,
            open_issues_count: 0,
            subscribers_count: 0,
            last_commit_at: Some(Utc::now()),
            structural: StructuralSignals::default(),
        }
    }

    fn categories(meta: GitHubRepoMetadata) -> Vec<String> {
        classify_repo(&meta)
            .into_iter()
            .map(|category| category.category)
            .collect()
    }

    fn categories_with_readme(meta: GitHubRepoMetadata, readme: &str) -> Vec<String> {
        classify_repo_with_readme(&meta, Some(readme))
            .into_iter()
            .map(|category| category.category)
            .collect()
    }

    #[test]
    fn classifies_ui_kits_from_names_and_topics() {
        let found = categories(meta(
            "shadcn-ui",
            "ui",
            "Beautifully designed components",
            &["components", "react", "tailwind"],
        ));

        assert!(found.contains(&"ui-kit".to_string()));
    }

    #[test]
    fn classifies_orm_repos() {
        let found = categories(meta(
            "prisma",
            "prisma",
            "Next-generation ORM for databases",
            &["orm", "database", "postgresql"],
        ));

        assert!(found.contains(&"orm".to_string()));
    }

    #[test]
    fn classifies_auth_repos() {
        let found = categories(meta(
            "nextauthjs",
            "next-auth",
            "Authentication for Next.js",
            &["oauth", "session", "security"],
        ));

        assert!(found.contains(&"auth".to_string()));
    }

    #[test]
    fn classifies_data_grid_repos() {
        let found = categories(meta(
            "TanStack",
            "table",
            "Headless UI for building datatables and grids",
            &["table", "datatable", "react"],
        ));

        assert!(found.contains(&"data-grid".to_string()));
    }

    #[test]
    fn leaves_unclear_repos_uncategorized() {
        let found = classify_repo(&meta("example", "misc", "Small helper", &["utility"]));

        assert!(found.is_empty());
    }

    #[test]
    fn readme_signals_can_classify_unclear_metadata() {
        let found = classify_repo_with_readme(
            &meta("example", "kit", "Small helper", &["react"]),
            Some("# Design system\n\nAccessible components for Tailwind applications."),
        );

        let category = found
            .iter()
            .find(|category| category.category == "ui-kit")
            .expect("README signals should classify UI kits");
        assert_eq!(category.source, "github_metadata+readme");
        assert_eq!(category.evidence["readmeMatched"][0], "components");
    }

    #[test]
    fn readme_evidence_does_not_store_raw_readme_text() {
        let raw_readme = "# Secretly long README\n\nHeadless UI components for a design system.";
        let found = classify_repo_with_readme(&meta("example", "kit", "", &[]), Some(raw_readme));

        let serialized = serde_json::to_string(&found).unwrap();
        assert!(!serialized.contains("Secretly long README"));
        assert!(!serialized.contains("Headless UI components for a design system"));
    }

    #[test]
    fn readme_generic_ui_words_do_not_create_ui_kit_false_positive() {
        let found = categories_with_readme(
            meta(
                "fastapi",
                "fastapi",
                "FastAPI framework with swagger-ui",
                &["framework", "swagger-ui"],
            ),
            "# FastAPI\n\nBuild APIs with docs, UI, interfaces and reusable components.",
        );

        assert!(!found.contains(&"ui-kit".to_string()));
    }

    #[test]
    fn readme_generic_test_word_does_not_create_testing_false_positive() {
        let found = categories_with_readme(
            meta(
                "axios",
                "axios",
                "Promise based HTTP client",
                &["http-client"],
            ),
            "# Axios\n\nRun tests before submitting a request change.",
        );

        assert!(!found.contains(&"testing".to_string()));
    }

    #[test]
    fn readme_generic_table_word_does_not_create_data_grid_false_positive() {
        let found = categories_with_readme(
            meta(
                "prisma",
                "prisma",
                "Next-generation ORM",
                &["orm", "database"],
            ),
            "# Prisma\n\nSee the compatibility table for database providers.",
        );

        assert!(!found.contains(&"data-grid".to_string()));
    }

    #[test]
    fn avoids_substring_false_positives() {
        let found = categories(meta(
            "fastapi",
            "fastapi",
            "FastAPI framework with json schema and swagger-ui",
            &["framework", "json-schema", "swagger-ui"],
        ));

        assert!(!found.contains(&"orm".to_string()));
        assert!(!found.contains(&"ui-kit".to_string()));
    }
}
