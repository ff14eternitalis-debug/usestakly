use sqlx::PgPool;
use uuid::Uuid;

use crate::app::error::ApiError;

use super::model::ExistingChannelSecretRow;

pub(crate) async fn existing_channel_secret(
    db: &PgPool,
    user_id: Uuid,
    channel_type: &str,
) -> Result<ExistingChannelSecretRow, ApiError> {
    sqlx::query_as(
        r#"
        SELECT destination, secret_ciphertext
        FROM notification_channels
        WHERE user_id = $1 AND channel_type = $2
        "#,
    )
    .bind(user_id)
    .bind(channel_type)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::bad_request("webhookUrl is required"))
}

pub(crate) async fn update_delivery_error(
    db: &PgPool,
    channel_id: Uuid,
    message: &str,
) -> Result<(), ApiError> {
    sqlx::query(
        "UPDATE notification_channels SET last_error = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(channel_id)
    .bind(message.chars().take(240).collect::<String>())
    .execute(db)
    .await?;
    Ok(())
}
