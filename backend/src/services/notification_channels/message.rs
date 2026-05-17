use serde_json::json;

use crate::domain::watchlist::NotificationKind;

#[derive(Debug, PartialEq)]
pub(crate) struct WatchAlertMessage {
    pub title: String,
    pub content: String,
    pub description: String,
    pub color: u32,
    pub fields: Vec<serde_json::Value>,
}

pub(crate) fn watch_alert_message(
    repo_full_name: &str,
    kind: NotificationKind,
    payload: &serde_json::Value,
) -> WatchAlertMessage {
    match kind {
        NotificationKind::ScoreDrop => {
            let prev = payload
                .get("prev_overall")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or_default();
            let new = payload
                .get("new_overall")
                .and_then(serde_json::Value::as_f64)
                .unwrap_or_default();
            WatchAlertMessage {
                title: format!("{repo_full_name}: score drop"),
                content: format!("UseStakly alert: {repo_full_name} quality score dropped."),
                description: "A watched repository crossed the score-drop alert threshold."
                    .to_string(),
                color: 16_744_996,
                fields: vec![
                    json!({ "name": "Previous score", "value": format!("{prev:.2}"), "inline": true }),
                    json!({ "name": "New score", "value": format!("{new:.2}"), "inline": true }),
                ],
            }
        }
        NotificationKind::AbandonmentUp => WatchAlertMessage {
            title: format!("{repo_full_name}: abandonment risk up"),
            content: format!("UseStakly alert: {repo_full_name} abandonment risk increased."),
            description: "A watched repository shows a higher abandonment risk.".to_string(),
            color: 16_744_996,
            fields: vec![],
        },
        NotificationKind::FlagAdded | NotificationKind::FlagSevere => {
            let flag = payload
                .get("flag")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("new flag");
            let severe = matches!(kind, NotificationKind::FlagSevere);
            WatchAlertMessage {
                title: format!("{repo_full_name}: {flag}"),
                content: format!("UseStakly alert: {repo_full_name} received flag `{flag}`."),
                description: if severe {
                    "A severe flag was detected on a watched repository."
                } else {
                    "A new flag was detected on a watched repository."
                }
                .to_string(),
                color: if severe { 15_115_908 } else { 16_744_996 },
                fields: vec![json!({ "name": "Flag", "value": flag, "inline": true })],
            }
        }
        NotificationKind::UseCaseNewCandidate => WatchAlertMessage {
            title: format!("{repo_full_name}: new radar candidate"),
            content: format!("UseStakly alert: {repo_full_name} entered a watched need."),
            description: "A repository entered the recommendations for a watched need.".to_string(),
            color: 8_900_331,
            fields: vec![],
        },
        NotificationKind::UseCaseBestCandidateChanged => WatchAlertMessage {
            title: format!("{repo_full_name}: best radar candidate changed"),
            content: format!(
                "UseStakly alert: {repo_full_name} is now the top match for a watched need."
            ),
            description: "The leading recommendation changed for a watched need.".to_string(),
            color: 8_900_331,
            fields: vec![],
        },
        NotificationKind::UseCaseQualityDrop => WatchAlertMessage {
            title: format!("{repo_full_name}: radar candidate quality dropped"),
            content: format!("UseStakly alert: {repo_full_name} dropped in a watched need."),
            description: "A repository in a watched need crossed the quality-drop threshold."
                .to_string(),
            color: 16_744_996,
            fields: vec![],
        },
        NotificationKind::UseCaseFlagAdded => {
            let flag = payload
                .get("flag")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("new flag");
            WatchAlertMessage {
                title: format!("{repo_full_name}: radar candidate flag"),
                content: format!(
                    "UseStakly alert: {repo_full_name} received flag `{flag}` in a watched need."
                ),
                description: "A repository in a watched need received a new flag.".to_string(),
                color: 16_744_996,
                fields: vec![json!({ "name": "Flag", "value": flag, "inline": true })],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn watch_alert_message_explains_score_drop() {
        let message = watch_alert_message(
            "facebook/react",
            NotificationKind::ScoreDrop,
            &json!({
                "prev_overall": 0.84,
                "new_overall": 0.68,
            }),
        );

        assert_eq!(message.title, "facebook/react: score drop");
        assert!(message.content.contains("facebook/react"));
        assert!(message.description.contains("score-drop alert threshold"));
        assert_eq!(message.fields.len(), 2);
    }
}
