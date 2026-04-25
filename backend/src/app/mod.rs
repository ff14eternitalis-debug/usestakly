pub mod error;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::AppConfig,
    handlers::{
        account, admin, agent_tokens, auth, health, me, notifications, repos, search, watchlist,
    },
    mcp::server as mcp_server,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
}

pub fn build_app(config: AppConfig, db: PgPool) -> Router {
    let frontend_origin = config
        .frontend_base_url
        .parse::<HeaderValue>()
        .unwrap_or_else(|_| HeaderValue::from_static("http://localhost:5173"));
    let state = AppState { config, db };
    let mcp_service = mcp_server::build_service(state.clone());

    Router::new()
        .route_service("/mcp", mcp_service)
        .route("/health", get(health::health))
        .route("/api/status/public", get(health::public_status))
        .route("/api/auth/github/start", get(auth::github_start))
        .route("/api/auth/github/callback", get(auth::github_callback))
        .route("/api/auth/discord/start", get(auth::discord_start))
        .route("/api/auth/discord/callback", get(auth::discord_callback))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/me", get(me::me))
        .route("/api/account/summary", get(account::account_summary))
        .route(
            "/api/admin/scoring/recompute",
            post(admin::recompute_scores),
        )
        .route(
            "/api/admin/scoring/explain/{repo_id}",
            get(admin::explain_scoring),
        )
        .route("/api/admin/ingest/github", post(admin::ingest_github_repo))
        .route(
            "/api/admin/embeddings/backfill",
            post(admin::backfill_repo_embeddings),
        )
        .route("/api/admin/mcp/metrics", get(admin::mcp_metrics_report))
        .route(
            "/api/admin/repo-signals/pending",
            get(admin::list_pending_repo_signals),
        )
        .route(
            "/api/admin/repo-signals/{signal_id}/review",
            post(admin::review_repo_signal),
        )
        .route("/api/search", get(search::search))
        .route("/api/repos/add", post(repos::add_repo))
        .route("/api/repos/search", get(repos::search_repos))
        .route("/api/repos/{repo_id}", get(repos::get_repo))
        .route(
            "/api/repos/{repo_id}/viewer-state",
            get(repos::get_repo_viewer_state),
        )
        .route(
            "/api/repos/{repo_id}/signals",
            post(repos::create_repo_signal),
        )
        .route(
            "/api/repos/{repo_id}/signals/{signal_id}/dispute",
            post(repos::dispute_repo_signal),
        )
        .route(
            "/api/watchlist",
            get(watchlist::list_watchlist).post(watchlist::add_to_watchlist),
        )
        .route(
            "/api/watchlist/{artifact_id}",
            axum::routing::patch(watchlist::update_watch).delete(watchlist::remove_from_watchlist),
        )
        .route("/api/notifications", get(notifications::list_notifications))
        .route(
            "/api/notifications/unread-count",
            get(notifications::unread_count),
        )
        .route(
            "/api/notifications/read-all",
            post(notifications::mark_all_read),
        )
        .route(
            "/api/notifications/{notification_id}/read",
            post(notifications::mark_notification_read),
        )
        .route(
            "/api/agent-tokens",
            get(agent_tokens::list_agent_tokens).post(agent_tokens::create_agent_token),
        )
        .route(
            "/api/agent-tokens/{token_id}",
            axum::routing::delete(agent_tokens::revoke_agent_token),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(frontend_origin)
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
