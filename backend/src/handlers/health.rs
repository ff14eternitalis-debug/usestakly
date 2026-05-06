use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{app::AppState, services::quality::load_v2};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicStatusResponse {
    status: &'static str,
    api: CheckStatus,
    database: CheckStatus,
    registry: RegistryStatus,
    mcp: McpStatus,
    formula: FormulaStatus,
    checked_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatus {
    status: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryStatus {
    status: &'static str,
    repo_count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpStatus {
    status: &'static str,
    tools: Vec<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormulaStatus {
    status: &'static str,
    version: String,
}

fn public_mcp_tools() -> Vec<&'static str> {
    vec![
        "recommend_github_repos",
        "search_github_repos",
        "get_repo_quality_context",
        "log_usage",
        "watch_repo",
        "watch_use_case",
    ]
}

pub async fn public_status(State(state): State<AppState>) -> Json<PublicStatusResponse> {
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();
    let repo_count = if db_ok {
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM external_artifacts WHERE source = 'github'",
        )
        .fetch_one(&state.db)
        .await
        .unwrap_or_default()
    } else {
        0
    };
    let formula_version = load_v2()
        .map(|formula| formula.meta.version)
        .unwrap_or_else(|_| "unavailable".to_string());
    let registry_ok = db_ok && repo_count > 0;
    let formula_ok = formula_version != "unavailable";
    let overall_ok = db_ok && registry_ok && formula_ok;

    Json(PublicStatusResponse {
        status: if overall_ok { "ok" } else { "degraded" },
        api: CheckStatus { status: "ok" },
        database: CheckStatus {
            status: if db_ok { "ok" } else { "down" },
        },
        registry: RegistryStatus {
            status: if registry_ok { "ok" } else { "degraded" },
            repo_count,
        },
        mcp: McpStatus {
            status: "ok",
            tools: public_mcp_tools(),
        },
        formula: FormulaStatus {
            status: if formula_ok { "ok" } else { "degraded" },
            version: formula_version,
        },
        checked_at: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::public_mcp_tools;

    #[test]
    fn public_status_lists_watch_use_case_tool() {
        assert!(public_mcp_tools().contains(&"watch_use_case"));
    }
}
