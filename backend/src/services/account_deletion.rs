use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::error::ApiError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAccountOutcome {
    pub revoked_tokens: u64,
    pub deleted_watchlist_rows: u64,
    pub deleted_notifications: u64,
    pub deleted_channels: u64,
    pub deleted_digest_deliveries: u64,
    pub deleted_use_case_watch_matches: u64,
    pub deleted_use_case_watches: u64,
    pub deleted_use_case_queries: u64,
    pub deleted_repo_refresh_events: u64,
    pub deleted_auth_identities: u64,
    pub anonymized_user: bool,
}

pub async fn delete_account_data(
    db: &PgPool,
    user_id: Uuid,
) -> Result<DeleteAccountOutcome, ApiError> {
    let mut tx = db.begin().await?;

    let revoked_tokens = sqlx::query(
        r#"
        UPDATE agent_tokens
        SET revoked_at = COALESCE(revoked_at, NOW()),
            label = 'deleted-token'
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?
    .rows_affected();

    let deleted_watchlist_rows = delete_by_user(
        &mut tx,
        "DELETE FROM watched_artifacts WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_notifications = delete_by_user(
        &mut tx,
        "DELETE FROM notifications WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_digest_deliveries = delete_by_user(
        &mut tx,
        "DELETE FROM notification_digest_deliveries WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_channels = delete_by_user(
        &mut tx,
        "DELETE FROM notification_channels WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_use_case_watch_matches = sqlx::query(
        r#"
        DELETE FROM use_case_watch_matches
        WHERE use_case_watch_id IN (
          SELECT id FROM use_case_watches WHERE user_id = $1
        )
        "#,
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    let deleted_use_case_watches = delete_by_user(
        &mut tx,
        "DELETE FROM use_case_watches WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_use_case_queries = delete_by_user(
        &mut tx,
        "DELETE FROM use_case_queries WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_repo_refresh_events = delete_by_user(
        &mut tx,
        "DELETE FROM repo_refresh_events WHERE user_id = $1",
        user_id,
    )
    .await?;
    let deleted_auth_identities = delete_by_user(
        &mut tx,
        "DELETE FROM auth_identities WHERE user_id = $1",
        user_id,
    )
    .await?;

    let anonymized_user = sqlx::query(
        r#"
        UPDATE users
        SET email = $2,
            username = $3,
            display_name = 'Deleted user',
            avatar_url = NULL,
            bio = NULL,
            digest_time_local = '08:00',
            timezone = 'UTC',
            email_locale = 'en',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(deleted_email_for(user_id))
    .bind(deleted_username_for(user_id))
    .execute(&mut *tx)
    .await?
    .rows_affected()
        == 1;

    tx.commit().await?;

    Ok(DeleteAccountOutcome {
        revoked_tokens,
        deleted_watchlist_rows,
        deleted_notifications,
        deleted_channels,
        deleted_digest_deliveries,
        deleted_use_case_watch_matches,
        deleted_use_case_watches,
        deleted_use_case_queries,
        deleted_repo_refresh_events,
        deleted_auth_identities,
        anonymized_user,
    })
}

async fn delete_by_user(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    query: &str,
    user_id: Uuid,
) -> Result<u64, ApiError> {
    Ok(sqlx::query(query)
        .bind(user_id)
        .execute(&mut **tx)
        .await?
        .rows_affected())
}

pub fn deleted_email_for(user_id: Uuid) -> String {
    format!("deleted+{user_id}@deleted.usestakly.local")
}

pub fn deleted_username_for(user_id: Uuid) -> String {
    format!("deleted-user-{}", user_id.simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_tombstone_email() {
        let user_id = Uuid::parse_str("019735da-1111-7222-8aaa-111111111111").unwrap();
        assert_eq!(
            deleted_email_for(user_id),
            "deleted+019735da-1111-7222-8aaa-111111111111@deleted.usestakly.local"
        );
    }

    #[test]
    fn builds_unique_tombstone_username() {
        let user_id = Uuid::parse_str("019735da-1111-7222-8aaa-111111111111").unwrap();
        assert_eq!(
            deleted_username_for(user_id),
            "deleted-user-019735da111172228aaa111111111111"
        );
    }
}
