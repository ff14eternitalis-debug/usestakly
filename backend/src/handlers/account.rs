use axum::{Json, extract::State, http::HeaderMap};

use crate::{
    app::{AppState, error::ApiError},
    auth::resolve_current_user,
    domain::account::AccountSummary,
    services::trust::reputation,
};

pub async fn account_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<AccountSummary>, ApiError> {
    let user = resolve_current_user(&state.db, &state.config, &headers).await?;
    let reputation = reputation::get_user_reputation(&state.db, user.id).await?;

    Ok(Json(AccountSummary {
        reputation: reputation.to_summary(state.config.active_signal_min_reputation),
        active_signal_min_reputation: state.config.active_signal_min_reputation,
        active_signal_default_consensus: state.config.active_signal_default_consensus,
        active_signal_severe_consensus: state.config.active_signal_severe_consensus,
    }))
}
