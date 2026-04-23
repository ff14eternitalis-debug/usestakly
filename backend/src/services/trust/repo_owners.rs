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
    let Some(repo_identity) = github_repo_identity(db, repo_id).await? else {
        return Ok(false);
    };

    if user_login.eq_ignore_ascii_case(&repo_identity.owner) {
        return Ok(true);
    }

    if is_public_org_member(config, &repo_identity.owner, &user_login).await? {
        return Ok(true);
    }
    if is_private_org_member(config, &repo_identity.owner, &user_login).await? {
        return Ok(true);
    }
    if has_repo_write_permissions(
        config,
        &repo_identity.owner,
        &repo_identity.name,
        &user_login,
    )
    .await?
    {
        return Ok(true);
    }

    Ok(false)
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

async fn github_repo_identity(
    db: &PgPool,
    repo_id: Uuid,
) -> Result<Option<GithubRepoIdentity>, ApiError> {
    let row: Option<GithubRepoIdentity> = sqlx::query_as(
        r#"
        SELECT github_owner AS owner, github_repo AS name
        FROM external_artifacts
        WHERE id = $1
          AND source = 'github'
          AND github_owner IS NOT NULL
          AND github_repo IS NOT NULL
        "#,
    )
    .bind(repo_id)
    .fetch_optional(db)
    .await?;

    Ok(row)
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
    let mut request = client
        .get(url)
        .header("Accept", "application/vnd.github+json");
    if let Some(token) = config.github_token.as_deref() {
        request = request.bearer_auth(token);
    }

    let response = request.send().await.map_err(|err| {
        ApiError::bad_request(format!("GitHub org membership lookup failed: {err}"))
    })?;

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

async fn is_private_org_member(
    config: &AppConfig,
    org: &str,
    login: &str,
) -> Result<bool, ApiError> {
    let Some(token) = config.github_token.as_deref() else {
        return Ok(false);
    };

    let client = github_client()?;
    let url = format!(
        "https://api.github.com/orgs/{}/memberships/{}",
        org.trim(),
        login.trim()
    );
    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|err| {
            ApiError::bad_request(format!("GitHub private membership lookup failed: {err}"))
        })?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::NOT_FOUND | StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED => Ok(false),
        StatusCode::TOO_MANY_REQUESTS => Err(ApiError::forbidden(
            "GitHub private org membership lookup is temporarily unavailable",
        )),
        other => Err(ApiError::bad_request(format!(
            "Unexpected GitHub private membership response: {other}"
        ))),
    }
}

async fn has_repo_write_permissions(
    config: &AppConfig,
    owner: &str,
    repo: &str,
    login: &str,
) -> Result<bool, ApiError> {
    let Some(token) = config.github_token.as_deref() else {
        return Ok(false);
    };

    let client = github_client()?;
    let url = format!(
        "https://api.github.com/repos/{}/{}/collaborators/{}/permission",
        owner.trim(),
        repo.trim(),
        login.trim()
    );
    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|err| {
            ApiError::bad_request(format!(
                "GitHub collaborator permission lookup failed: {err}"
            ))
        })?;

    match response.status() {
        StatusCode::OK => {
            let body = response
                .json::<CollaboratorPermissionResponse>()
                .await
                .map_err(|err| {
                    ApiError::bad_request(format!(
                        "Invalid GitHub collaborator permission response: {err}"
                    ))
                })?;
            Ok(
                matches!(body.permission.as_deref(), Some("admin") | Some("write"))
                    || matches!(
                        body.role_name.as_deref(),
                        Some("admin") | Some("write") | Some("maintain")
                    ),
            )
        }
        StatusCode::NOT_FOUND | StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED => Ok(false),
        StatusCode::TOO_MANY_REQUESTS => Err(ApiError::forbidden(
            "GitHub collaborator permission lookup is temporarily unavailable",
        )),
        other => Err(ApiError::bad_request(format!(
            "Unexpected GitHub collaborator permission response: {other}"
        ))),
    }
}

fn github_client() -> Result<Client, ApiError> {
    Client::builder()
        .user_agent("UseStakly-MVP")
        .build()
        .map_err(|err| ApiError::internal(format!("failed to build GitHub client: {err}")))
}

#[derive(sqlx::FromRow)]
struct GithubRepoIdentity {
    owner: String,
    name: String,
}

#[derive(serde::Deserialize)]
struct CollaboratorPermissionResponse {
    permission: Option<String>,
    role_name: Option<String>,
}
