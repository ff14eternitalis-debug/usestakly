use axum::http::HeaderMap;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig};

const SESSION_COOKIE_NAME: &str = "usestakly_session";
const GITHUB_PROVIDER: &str = "github";
const DEBUG_USER_ID_HEADER: &str = "x-debug-user-id";
const DEBUG_USER_EMAIL_HEADER: &str = "x-debug-user-email";
const DEBUG_USER_USERNAME_HEADER: &str = "x-debug-user-username";
const DEBUG_USER_DISPLAY_NAME_HEADER: &str = "x-debug-user-display-name";
const DEBUG_USER_AVATAR_URL_HEADER: &str = "x-debug-user-avatar-url";

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionClaims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OAuthStateClaims {
    nonce: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct GithubTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubUser {
    id: u64,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn resolve_current_user(
    db: &PgPool,
    config: &AppConfig,
    headers: &HeaderMap,
) -> Result<CurrentUser, ApiError> {
    if let Some(user) = current_user_from_session(db, config, headers).await? {
        return Ok(user);
    }

    if config.github_auth_enabled() {
        return Err(ApiError::unauthorized("Authentication required"));
    }

    resolve_dev_user(db, config, headers).await
}

pub fn github_oauth_url(config: &AppConfig) -> Result<String, ApiError> {
    let client_id = config
        .github_client_id
        .as_ref()
        .ok_or_else(|| ApiError::bad_request("GitHub auth is not configured"))?;
    let state = encode_oauth_state(config)?;

    Ok(format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=read:user%20user:email&state={}&allow_signup=true",
        client_id,
        urlencoding::encode(&config.github_callback_url()),
        urlencoding::encode(&state),
    ))
}

pub async fn finish_github_oauth(
    db: &PgPool,
    config: &AppConfig,
    code: &str,
    state: &str,
) -> Result<CurrentUser, ApiError> {
    validate_oauth_state(config, state)?;

    let client_id = config
        .github_client_id
        .clone()
        .ok_or_else(|| ApiError::bad_request("GitHub client id missing"))?;
    let client_secret = config
        .github_client_secret
        .clone()
        .ok_or_else(|| ApiError::bad_request("GitHub client secret missing"))?;

    let http = Client::new();
    let token_response = http
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code),
            ("redirect_uri", config.github_callback_url().as_str()),
        ])
        .send()
        .await
        .map_err(|err| ApiError::bad_request(format!("GitHub token exchange failed: {err}")))?;

    let token_body = token_response
        .json::<GithubTokenResponse>()
        .await
        .map_err(|err| ApiError::bad_request(format!("Invalid GitHub token response: {err}")))?;

    let access_token = token_body.access_token.ok_or_else(|| {
        ApiError::bad_request(
            token_body
                .error_description
                .or(token_body.error)
                .unwrap_or_else(|| "GitHub did not return an access token".to_string()),
        )
    })?;

    let github_user = http
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "UseStakly-MVP")
        .send()
        .await
        .map_err(|err| ApiError::bad_request(format!("GitHub user request failed: {err}")))?
        .error_for_status()
        .map_err(|err| ApiError::bad_request(format!("GitHub user request failed: {err}")))?
        .json::<GithubUser>()
        .await
        .map_err(|err| ApiError::bad_request(format!("Invalid GitHub user response: {err}")))?;

    let github_emails = http
        .get("https://api.github.com/user/emails")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "UseStakly-MVP")
        .send()
        .await
        .map_err(|err| ApiError::bad_request(format!("GitHub email request failed: {err}")))?
        .error_for_status()
        .map_err(|err| ApiError::bad_request(format!("GitHub email request failed: {err}")))?
        .json::<Vec<GithubEmail>>()
        .await
        .map_err(|err| ApiError::bad_request(format!("Invalid GitHub email response: {err}")))?;

    let primary_email = github_emails
        .iter()
        .find(|email| email.primary && email.verified)
        .map(|email| email.email.clone())
        .or_else(|| {
            github_emails
                .iter()
                .find(|email| email.verified)
                .map(|email| email.email.clone())
        })
        .or(github_user.email.clone())
        .ok_or_else(|| {
            ApiError::bad_request(
                "GitHub account must expose at least one verified email for MVP login",
            )
        })?;

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, username, display_name, avatar_url)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (email) DO UPDATE
        SET
          username = EXCLUDED.username,
          display_name = EXCLUDED.display_name,
          avatar_url = EXCLUDED.avatar_url,
          updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(&primary_email)
    .bind(&github_user.login)
    .bind(&github_user.name)
    .bind(&github_user.avatar_url)
    .fetch_one(db)
    .await
    .map_err(ApiError::from)?;

    sqlx::query(
        r#"
        INSERT INTO auth_identities (user_id, provider, provider_user_id, credentials)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (provider, provider_user_id) DO UPDATE
        SET
          user_id = EXCLUDED.user_id,
          credentials = EXCLUDED.credentials
        "#,
    )
    .bind(user_id)
    .bind(GITHUB_PROVIDER)
    .bind(github_user.id.to_string())
    .bind(serde_json::json!({
        "login": github_user.login,
        "email": primary_email,
    }))
    .execute(db)
    .await
    .map_err(ApiError::from)?;

    Ok(CurrentUser {
        id: user_id,
        email: primary_email,
        username: github_user.login,
        display_name: github_user.name,
        avatar_url: github_user.avatar_url,
    })
}

pub fn session_cookie(config: &AppConfig, user_id: Uuid) -> Result<String, ApiError> {
    let token = encode_session_token(config, user_id)?;
    Ok(format!(
        "{name}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000{secure}",
        name = SESSION_COOKIE_NAME,
        secure = if config.session_cookie_secure() {
            "; Secure"
        } else {
            ""
        }
    ))
}

pub fn clear_session_cookie(config: &AppConfig) -> Result<String, ApiError> {
    Ok(format!(
        "{name}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{secure}",
        name = SESSION_COOKIE_NAME,
        secure = if config.session_cookie_secure() {
            "; Secure"
        } else {
            ""
        }
    ))
}

async fn current_user_from_session(
    db: &PgPool,
    config: &AppConfig,
    headers: &HeaderMap,
) -> Result<Option<CurrentUser>, ApiError> {
    let token = match cookie_value(headers, SESSION_COOKIE_NAME) {
        Some(token) => token,
        None => return Ok(None),
    };

    let claims = decode_session_token(config, &token)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::unauthorized("Invalid session subject"))?;

    let user = sqlx::query_as::<_, CurrentUser>(
        r#"
        SELECT id, email, username, display_name, avatar_url
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await
    .map_err(ApiError::from)?;

    Ok(user)
}

async fn resolve_dev_user(
    db: &PgPool,
    config: &AppConfig,
    headers: &HeaderMap,
) -> Result<CurrentUser, ApiError> {
    let user_id = header_value(headers, DEBUG_USER_ID_HEADER)
        .map(|value| Uuid::parse_str(&value))
        .transpose()
        .map_err(|_| ApiError::bad_request("x-debug-user-id must be a valid UUID"))?
        .unwrap_or(config.dev_user_id);

    let email = header_value(headers, DEBUG_USER_EMAIL_HEADER)
        .unwrap_or_else(|| config.dev_user_email.clone());
    let username = header_value(headers, DEBUG_USER_USERNAME_HEADER)
        .unwrap_or_else(|| config.dev_user_username.clone());
    let display_name = header_value(headers, DEBUG_USER_DISPLAY_NAME_HEADER)
        .or_else(|| config.dev_user_display_name.clone());
    let avatar_url = header_value(headers, DEBUG_USER_AVATAR_URL_HEADER)
        .or_else(|| config.dev_user_avatar_url.clone());

    sqlx::query(
        r#"
        INSERT INTO users (id, email, username, display_name, avatar_url)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id) DO UPDATE
        SET
          email = EXCLUDED.email,
          username = EXCLUDED.username,
          display_name = EXCLUDED.display_name,
          avatar_url = EXCLUDED.avatar_url,
          updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(&email)
    .bind(&username)
    .bind(&display_name)
    .bind(&avatar_url)
    .execute(db)
    .await
    .map_err(ApiError::from)?;

    Ok(CurrentUser {
        id: user_id,
        email,
        username,
        display_name,
        avatar_url,
    })
}

fn encode_session_token(config: &AppConfig, user_id: Uuid) -> Result<String, ApiError> {
    let secret = config
        .app_session_secret
        .as_ref()
        .ok_or_else(|| ApiError::bad_request("APP_SESSION_SECRET is required"))?;
    let claims = SessionClaims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ApiError::bad_request("Unable to encode session token"))
}

fn decode_session_token(config: &AppConfig, token: &str) -> Result<SessionClaims, ApiError> {
    let secret = config
        .app_session_secret
        .as_ref()
        .ok_or_else(|| ApiError::unauthorized("Session secret missing"))?;
    let validation = Validation::new(Algorithm::HS256);
    decode::<SessionClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::unauthorized("Invalid session"))
}

fn encode_oauth_state(config: &AppConfig) -> Result<String, ApiError> {
    let secret = config
        .app_session_secret
        .as_ref()
        .ok_or_else(|| ApiError::bad_request("APP_SESSION_SECRET is required"))?;
    let claims = OAuthStateClaims {
        nonce: Uuid::new_v4().to_string(),
        exp: (Utc::now() + Duration::minutes(15)).timestamp() as usize,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ApiError::bad_request("Unable to encode oauth state"))
}

fn validate_oauth_state(config: &AppConfig, state: &str) -> Result<(), ApiError> {
    let secret = config
        .app_session_secret
        .as_ref()
        .ok_or_else(|| ApiError::bad_request("APP_SESSION_SECRET is required"))?;
    let validation = Validation::new(Algorithm::HS256);
    decode::<OAuthStateClaims>(
        state,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| ApiError::bad_request("Invalid OAuth state"))?;
    Ok(())
}

fn header_value(headers: &HeaderMap, name: &'static str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(axum::http::header::COOKIE)?.to_str().ok()?;
    raw.split(';')
        .filter_map(|part| part.trim().split_once('='))
        .find_map(|(cookie_name, cookie_value)| {
            (cookie_name == name).then(|| cookie_value.to_string())
        })
}
