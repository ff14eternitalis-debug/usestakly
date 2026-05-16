pub mod capture;
pub mod compute;
pub mod dimension_state;
pub mod flags;
pub mod formula;
pub mod ingestion_status;
pub mod pipeline;
pub mod weighting;

pub use capture::{RecordSignalInput, record_signal};
pub use compute::{ArtifactMetrics, ComputedScore, compute_score};
pub use dimension_state::{
    DimensionStateInput, build_dimension_states, derive_proof_tier, proof_tier_str,
};
pub use flags::ConsensusThresholds;
pub use formula::{Formula, load_v1, load_v2};
pub use ingestion_status::build_ingestion_status;
pub use pipeline::{
    ScoringExplain, ScoringReport, explain_external_scoring, recompute_all_scores,
    recompute_all_scores_with_config, recompute_external_artifact,
};
pub use weighting::{SignalObservation, SignalWeightBreakdown, WeightedCounts};
