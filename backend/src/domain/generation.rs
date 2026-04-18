use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyStep {
    pub domain: String,
    pub reason: String,
}
