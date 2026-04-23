use reqwest::{Client, StatusCode};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app::error::ApiError, config::AppConfig};

pub async fn user_can_manage_repo_signal(
    db: &PgPool,
    config: &AppConfig,
    user_id: Uuid,
    repo_id: Uuid,
) -> Result<bool, ApiError> {
    let Some(user_login) = github_login_for_user(db, user_id).await? else {
        return Ok(false);
    };
    let Some(repo_owner) = github_owner_for_repo(db, repo_id).await? else {
        return Ok(false);
    };

    if user_login.eq_ignore_ascii_case(&repo_owner) {
        return Ok(true);
    }

    is_public_org_member(config, &repo_owner, &user_login).await
}

async fn github_login_for_user(db: &PgPool, user_id: Uuid) -> Result<Option<String>, ApiError> {
    let login: Option<String> = sqlx::query_scalar(
        r#"
        SELECT ai.credentials->>'login'
        FROM auth_identities ai
        WHERE ai.user_id = $1
          AND ai.provider = 'github'
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    Ok(login.filter(|value| !value.trim().is_empty()))
}

async fn github_owner_for_repo(db: &PgPool, repo_id: Uuid) -> Result<Option<String>, ApiError> {
    let owner: Option<String> = sqlx::query_scalar(
        r#"
        SELECT github_owner
        FROM external_artifacts
        WHERE id = $1
          AND source = 'github'
        "#,
    )
    .bind(repo_id)
    .fetch_optional(db)
    .await?;

    Ok(owner.filter(|value| !value.trim().is_empty()))
}

async fn is_public_org_member(
    config: &AppConfig,
    org: &str,
    login: &str,
) -> Result<bool, ApiError> {
    let client = Client::builder()
        .user_agent("UseStakly-MVP")
        .build()
        .map_err(|err| ApiError::internal(format!("failed to build GitHub client: {err}")))?;

    let url = format!(
        "https://api.github.com/orgs/{}/public_members/{}",
        org.trim(),
        login.trim()
    );
    let mut request = client.get(url).header("Accept", "application/vnd.github+json");
    if let Some(token) = config.github_token.as_deref() {
        request = request.bearer_auth(token);
    }

    let response = request
        .send()
        .await
        .map_err(|err| ApiError::bad_request(format!("GitHub org membership lookup failed: {err}")))?;

    match response.status() {
        StatusCode::NO_CONTENT => Ok(true),
        StatusCode::NOT_FOUND => Ok(false),
        StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => Err(ApiError::forbidden(
            "GitHub org membership lookup is temporarily unavailable",
        )),
        other => Err(ApiError::bad_request(format!(
            "Unexpected GitHub org membership response: {other}"
        ))),
    }
}
