use axum::http::HeaderMap;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig};

const DEBUG_USER_ID_HEADER: &str = "x-debug-user-id";
const DEBUG_USER_EMAIL_HEADER: &str = "x-debug-user-email";
const DEBUG_USER_USERNAME_HEADER: &str = "x-debug-user-username";
const DEBUG_USER_DISPLAY_NAME_HEADER: &str = "x-debug-user-display-name";
const DEBUG_USER_AVATAR_URL_HEADER: &str = "x-debug-user-avatar-url";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn resolve_current_user(
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

fn header_value(headers: &HeaderMap, name: &'static str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}
