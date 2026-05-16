use chrono::{DateTime, Utc};

use crate::domain::{quality_display::IngestionStatus, repo::VitalityInputs};

pub fn build_ingestion_status(
    priors_fetched_at: Option<DateTime<Utc>>,
    vitality: &VitalityInputs,
    structural_stale_secs: u64,
    now: DateTime<Utc>,
) -> IngestionStatus {
    let structural_signals_at = vitality.structural_signals_at;
    let mut partial_fields = Vec::new();
    if structural_signals_at.is_none() {
        partial_fields.push("structural_signals".to_string());
    } else {
        if vitality.distinct_contributors_90d.is_none() {
            partial_fields.push("distinct_contributors_90d".to_string());
        }
        if vitality.commits_30d.is_none() {
            partial_fields.push("commits_30d".to_string());
        }
        if vitality.has_ci.is_none() {
            partial_fields.push("has_ci".to_string());
        }
        if vitality.releases_count.is_none() {
            partial_fields.push("releases_count".to_string());
        }
    }

    let structural_stale = structural_signals_at.is_none_or(|at| {
        (now - at).num_seconds() > structural_stale_secs as i64
    });
    let structural_complete = structural_signals_at.is_some() && partial_fields.is_empty();

    IngestionStatus {
        priors_fetched_at,
        structural_signals_at,
        structural_stale,
        structural_complete,
        partial_fields,
    }
}
