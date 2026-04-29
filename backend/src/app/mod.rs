pub mod error;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderValue, Method, Request, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    config::AppConfig,
    handlers::{
        account, admin, agent_tokens, auth, health, me, notifications, repos, search, use_cases,
        watchlist,
    },
    mcp::server as mcp_server,
    services::agent_tokens as agent_token_service,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
}

pub fn build_app(config: AppConfig, db: PgPool) -> Router {
    let allowed_origins = allowed_frontend_origins(&config.frontend_base_url);
    let state = AppState { config, db };
    let mcp_service = mcp_server::build_service(state.clone());
    let mcp_routes = Router::new()
        .route_service("/mcp", mcp_service)
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_mcp_authorization,
        ));

    Router::new()
        .merge(mcp_routes)
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
        .route(
            "/api/use-cases/recommend",
            post(use_cases::recommend_use_case),
        )
        .route(
            "/api/use-cases/watch",
            get(use_cases::list_use_case_watches).post(use_cases::create_use_case_watch),
        )
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
                .allow_origin(AllowOrigin::predicate(move |origin, _| {
                    allowed_origins.iter().any(|allowed| allowed == origin)
                }))
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

fn allowed_frontend_origins(frontend_base_url: &str) -> Vec<HeaderValue> {
    let mut origins = Vec::new();
    push_origin(&mut origins, frontend_base_url);
    if frontend_base_url.starts_with("http://localhost:") {
        push_origin(
            &mut origins,
            &frontend_base_url.replacen("http://localhost:", "http://127.0.0.1:", 1),
        );
    } else if frontend_base_url.starts_with("http://127.0.0.1:") {
        push_origin(
            &mut origins,
            &frontend_base_url.replacen("http://127.0.0.1:", "http://localhost:", 1),
        );
    }
    if origins.is_empty() {
        origins.push(HeaderValue::from_static("http://localhost:5173"));
        origins.push(HeaderValue::from_static("http://127.0.0.1:5173"));
    }
    origins
}

fn push_origin(origins: &mut Vec<HeaderValue>, value: &str) {
    if let Ok(origin) = value.parse::<HeaderValue>()
        && !origins.contains(&origin)
    {
        origins.push(origin);
    }
}

async fn require_mcp_authorization(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let Some(raw) = request.headers().get(header::AUTHORIZATION) else {
        return (StatusCode::UNAUTHORIZED, "missing Authorization header").into_response();
    };
    let Ok(raw) = raw.to_str() else {
        return (StatusCode::UNAUTHORIZED, "malformed Authorization header").into_response();
    };
    let Some(token) = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
    else {
        return (StatusCode::UNAUTHORIZED, "expected 'Bearer <token>'").into_response();
    };

    match agent_token_service::verify(&state.db, token.trim()).await {
        Ok(_) => next.run(request).await,
        Err(_) => (StatusCode::UNAUTHORIZED, "invalid or revoked token").into_response(),
    }
}
