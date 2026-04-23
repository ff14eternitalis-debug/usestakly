use chrono::Utc;
use sqlx::{FromRow, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{app::error::ApiError, domain::repo::RepoSignalEvent};

pub async fn record_signal_event(
    db: &PgPool,
    signal_id: Uuid,
    event_kind: &str,
    actor_user_id: Option<Uuid>,
    note: Option<&str>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO quality_signal_events (quality_signal_id, event_kind, actor_user_id, note, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(signal_id)
    .bind(event_kind)
    .bind(actor_user_id)
    .bind(note)
    .bind(Utc::now())
    .execute(db)
    .await?;
    Ok(())
}

pub async fn list_events_for_signals(
    db: &PgPool,
    signal_ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<RepoSignalEvent>>, ApiError> {
    if signal_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows: Vec<SignalEventRow> = sqlx::query_as(
        r#"
        SELECT quality_signal_id, event_kind, note, created_at
        FROM quality_signal_events
        WHERE quality_signal_id = ANY($1)
        ORDER BY created_at DESC
        "#,
    )
    .bind(signal_ids)
    .fetch_all(db)
    .await?;

    let mut grouped: HashMap<Uuid, Vec<RepoSignalEvent>> = HashMap::new();
    for row in rows {
        grouped
            .entry(row.quality_signal_id)
            .or_default()
            .push(RepoSignalEvent {
                event_kind: row.event_kind,
                note: row.note,
                created_at: row.created_at,
            });
    }

    Ok(grouped)
}

#[derive(FromRow)]
struct SignalEventRow {
    quality_signal_id: Uuid,
    event_kind: String,
    note: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}
