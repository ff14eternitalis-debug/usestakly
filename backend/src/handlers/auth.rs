use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    app::{AppState, error::ApiError},
    auth::{
        clear_session_cookie, discord_oauth_url, finish_discord_oauth, finish_github_oauth,
        github_oauth_url, oauth_return_to, session_cookie,
    },
};

#[derive(Deserialize)]
pub struct GithubCallbackQuery {
    code: String,
    state: String,
}

#[derive(Deserialize)]
pub struct OAuthStartQuery {
    return_to: Option<String>,
}

pub async fn github_start(
    State(state): State<AppState>,
    Query(query): Query<OAuthStartQuery>,
) -> Result<Redirect, ApiError> {
    let url = github_oauth_url(&state.config, query.return_to.as_deref())?;
    Ok(Redirect::temporary(&url))
}

pub async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<GithubCallbackQuery>,
) -> Result<Response, ApiError> {
    let current_user =
        finish_github_oauth(&state.db, &state.config, &query.code, &query.state).await?;
    let cookie = session_cookie(&state.config, current_user.id)?;
    let return_to = oauth_return_to(&state.config, &query.state)?;

    let mut response =
        Redirect::to(&format!("{}{}", state.config.frontend_base_url, return_to)).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|_| ApiError::bad_request("invalid session cookie"))?,
    );
    Ok(response)
}

pub async fn discord_start(
    State(state): State<AppState>,
    Query(query): Query<OAuthStartQuery>,
) -> Result<Redirect, ApiError> {
    let url = discord_oauth_url(&state.config, query.return_to.as_deref())?;
    Ok(Redirect::temporary(&url))
}

pub async fn discord_callback(
    State(state): State<AppState>,
    Query(query): Query<GithubCallbackQuery>,
) -> Result<Response, ApiError> {
    let current_user =
        finish_discord_oauth(&state.db, &state.config, &query.code, &query.state).await?;
    let cookie = session_cookie(&state.config, current_user.id)?;
    let return_to = oauth_return_to(&state.config, &query.state)?;

    let mut response =
        Redirect::to(&format!("{}{}", state.config.frontend_base_url, return_to)).into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|_| ApiError::bad_request("invalid session cookie"))?,
    );
    Ok(response)
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let cookie = clear_session_cookie(&state.config)?;
    let mut response = (
        StatusCode::NO_CONTENT,
        Json(serde_json::json!({ "ok": true })),
    )
        .into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|_| ApiError::bad_request("invalid session cookie"))?,
    );
    let _ = headers;
    Ok(response)
}
