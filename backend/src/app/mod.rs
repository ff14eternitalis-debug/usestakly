pub mod error;
mod mcp_rate_limit;

use std::{sync::Arc, time::Instant};

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderMap, HeaderValue, Method, Request, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    app::mcp_rate_limit::{McpRateLimitKey, McpRateLimiter},
    config::AppConfig,
    handlers::{
        account, admin, agent_tokens, auth, health, me, notification_channels, notifications,
        repos, search, use_cases, watchlist,
    },
    mcp::server as mcp_server,
    services::agent_tokens as agent_token_service,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: PgPool,
    mcp_rate_limits: Arc<McpRateLimits>,
}

#[derive(Debug)]
struct McpRateLimits {
    auth_failures: McpRateLimiter,
    authenticated: McpRateLimiter,
}

pub fn build_app(config: AppConfig, db: PgPool) -> Router {
    let allowed_origins = allowed_frontend_origins(&config.frontend_base_url);
    let mcp_rate_limits = Arc::new(McpRateLimits {
        auth_failures: McpRateLimiter::per_minute(config.mcp_auth_failure_limit_per_minute),
        authenticated: McpRateLimiter::per_minute(config.mcp_read_limit_per_minute),
    });
    let state = AppState {
        config,
        db,
        mcp_rate_limits,
    };
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
        .route(
            "/api/account",
            axum::routing::delete(account::delete_account),
        )
        .route("/api/account/summary", get(account::account_summary))
        .route(
            "/api/account/notification-preferences",
            get(account::get_notification_preferences)
                .put(account::update_notification_preferences),
        )
        .route(
            "/api/account/notification-channels",
            get(notification_channels::list_notification_channels)
                .post(notification_channels::upsert_notification_channel),
        )
        .route(
            "/api/account/notification-channels/{channel_id}",
            axum::routing::delete(notification_channels::delete_notification_channel),
        )
        .route(
            "/api/account/notification-channels/{channel_id}/test",
            post(notification_channels::test_notification_channel),
        )
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
        .route("/api/admin/github/quota", get(admin::github_quota_report))
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
        .route("/api/repos/{repo_id}/refresh", post(repos::refresh_repo))
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
                    Method::PUT,
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
    let source_ip = source_ip(request.headers());
    let Some(raw) = request.headers().get(header::AUTHORIZATION) else {
        if let Err(retry_after) = state
            .mcp_rate_limits
            .auth_failures
            .check(McpRateLimitKey::InvalidAuthIp(source_ip), Instant::now())
        {
            return rate_limited_response(retry_after);
        }
        return (StatusCode::UNAUTHORIZED, "missing Authorization header").into_response();
    };
    let Ok(raw) = raw.to_str() else {
        if let Err(retry_after) = state
            .mcp_rate_limits
            .auth_failures
            .check(McpRateLimitKey::InvalidAuthIp(source_ip), Instant::now())
        {
            return rate_limited_response(retry_after);
        }
        return (StatusCode::UNAUTHORIZED, "malformed Authorization header").into_response();
    };
    let Some(token) = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
    else {
        if let Err(retry_after) = state
            .mcp_rate_limits
            .auth_failures
            .check(McpRateLimitKey::InvalidAuthIp(source_ip), Instant::now())
        {
            return rate_limited_response(retry_after);
        }
        return (StatusCode::UNAUTHORIZED, "expected 'Bearer <token>'").into_response();
    };

    let token = token.trim();
    let invalid_auth_key = McpRateLimitKey::InvalidAuthIp(source_ip);
    if let Some(retry_after) = state
        .mcp_rate_limits
        .auth_failures
        .is_limited(&invalid_auth_key, Instant::now())
    {
        return rate_limited_response(retry_after);
    }

    match agent_token_service::verify(&state.db, token).await {
        Ok(_) => {
            if let Err(retry_after) = state.mcp_rate_limits.authenticated.check(
                McpRateLimitKey::Token(token_fingerprint(token)),
                Instant::now(),
            ) {
                return rate_limited_response(retry_after);
            }
            next.run(request).await
        }
        Err(_) => {
            if let Err(retry_after) = state
                .mcp_rate_limits
                .auth_failures
                .check(invalid_auth_key, Instant::now())
            {
                return rate_limited_response(retry_after);
            }
            (StatusCode::UNAUTHORIZED, "invalid or revoked token").into_response()
        }
    }
}

fn source_ip(headers: &HeaderMap) -> String {
    headers
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.split(',').next())
        })
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("unknown")
        .to_string()
}

fn token_fingerprint(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

fn rate_limited_response(retry_after: std::time::Duration) -> Response {
    let retry_secs = retry_after.as_secs().max(1).to_string();
    let mut response = (StatusCode::TOO_MANY_REQUESTS, "MCP rate limit exceeded").into_response();
    if let Ok(value) = HeaderValue::from_str(&retry_secs) {
        response.headers_mut().insert(header::RETRY_AFTER, value);
    }
    response
}
