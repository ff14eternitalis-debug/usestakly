pub mod capture;
pub mod compute;
pub mod flags;
pub mod formula;
pub mod pipeline;
pub mod weighting;

pub use capture::{RecordSignalInput, record_signal};
pub use compute::{ArtifactMetrics, ComputedScore, compute_score};
pub use flags::ConsensusThresholds;
pub use formula::{Formula, load_v1};
pub use pipeline::{
    ScoringExplain, ScoringReport, explain_external_scoring, recompute_all_scores,
    recompute_all_scores_with_config,
};
pub use weighting::{SignalObservation, SignalWeightBreakdown, WeightedCounts};
