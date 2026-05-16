use super::common::RiskTolerance;
use super::search::radar_summary;

pub(crate) fn recommendation_reasons(
    quality: Option<&crate::domain::reference::QualityContext>,
    radar: Option<&crate::domain::repo::RepoRadarSnapshot>,
    risk: RiskTolerance,
) -> Vec<String> {
    let Some(q) = quality else {
        return vec!["No score is available yet; inspect the repo before adopting.".to_string()];
    };
    let mut reasons = Vec::new();
    if let Some(overall) = q.overall {
        reasons.push(format!("Overall dependency score is {:.3}.", overall));
    }
    if q.freshness.unwrap_or(0.0) >= 0.8 {
        reasons.push("Freshness is strong, indicating recent repository activity.".to_string());
    }
    if q.abandonment.unwrap_or(1.0) <= 0.2 {
        reasons.push("Abandonment risk is currently low.".to_string());
    }
    if q.reliability.unwrap_or(0.5) > 0.5 {
        reasons.push("Reliability is supported by positive usage outcomes.".to_string());
    } else if q.build_success_count > 0 || q.build_failure_count > 0 {
        reasons.push(format!(
            "Reliability has {} build success and {} build failure signals.",
            q.build_success_count, q.build_failure_count
        ));
    }
    if reasons.is_empty() {
        reasons.push(
            "Included because it matched the query and passed the selected filter.".to_string(),
        );
    }
    if let Some(radar) = radar {
        reasons.push(radar_summary(radar));
    }
    match risk {
        RiskTolerance::Low => reasons.push(
            "Low risk tolerance favored stricter quality gates and maintenance signals."
                .to_string(),
        ),
        RiskTolerance::High => reasons.push(
            "High risk tolerance allowed relevance to weigh more than mature usage history."
                .to_string(),
        ),
        RiskTolerance::Medium => {}
    }
    reasons
}

pub(crate) fn recommendation_caveats(
    quality: Option<&crate::domain::reference::QualityContext>,
    radar: Option<&crate::domain::repo::RepoRadarSnapshot>,
    risk: RiskTolerance,
) -> Vec<String> {
    let Some(q) = quality else {
        return vec!["Score provenance is missing until the repo is computed.".to_string()];
    };
    let mut caveats = Vec::new();
    if q.reliability.unwrap_or(0.5) == 0.5 && q.build_success_count + q.build_failure_count < 5 {
        caveats.push(
            "Reliability is still neutral because there are fewer than 5 build samples."
                .to_string(),
        );
    }
    if q.adoption.unwrap_or(0.0) == 0.0 && q.resolve_count == 0 {
        caveats.push(
            "Adoption has no usage outcomes yet; treat popularity separately from proven usage."
                .to_string(),
        );
    }
    if !q.flags.is_empty() {
        caveats.push(format!("Active flags to inspect: {}.", q.flags.join(", ")));
    }
    if q.abandonment.unwrap_or(0.0) > 0.4 {
        caveats
            .push("Abandonment risk is elevated; inspect maintenance before adoption.".to_string());
    }
    if let Some(radar) = radar
        && matches!(
            radar.maturity_band.as_str(),
            "emerging" | "experimental" | "noisy"
        )
    {
        caveats.push(format!(
            "Radar marks this repo as {}; validate fit before production adoption.",
            radar.maturity_band
        ));
    }
    if risk == RiskTolerance::High {
        caveats.push(
            "Because risk_tolerance is high, validate API stability and maintenance manually."
                .to_string(),
        );
    }
    caveats
}

pub(crate) fn recommendation_next_actions(
    quality: Option<&crate::domain::reference::QualityContext>,
    radar: Option<&crate::domain::repo::RepoRadarSnapshot>,
) -> Vec<String> {
    let mut actions = vec!["Call get_repo_quality_context before final selection.".to_string()];
    if quality
        .map(|q| q.resolve_count + q.build_success_count + q.build_failure_count < 5)
        .unwrap_or(true)
    {
        actions.push("Run a small install/build smoke test before recommending it.".to_string());
    }
    actions.push("After testing the dependency, call log_usage with the outcome.".to_string());
    actions.push("Use watch_repo if this becomes a dependency to monitor.".to_string());
    if radar
        .map(|radar| matches!(radar.maturity_band.as_str(), "emerging" | "experimental"))
        .unwrap_or(false)
    {
        actions.push(
            "If choosing an emerging repo, use watch_repo to monitor quality drift.".to_string(),
        );
    }
    actions
}
