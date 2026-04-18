use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchScope {
    #[serde(rename = "private_only")]
    PrivateOnly,
    #[serde(rename = "own_plus_public")]
    OwnPlusPublic,
    #[serde(rename = "public_only")]
    PublicOnly,
    #[serde(rename = "selected_libraries_only")]
    SelectedLibrariesOnly,
}
