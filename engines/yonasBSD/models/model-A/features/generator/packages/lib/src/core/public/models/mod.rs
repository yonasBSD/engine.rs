// ==========================================================
// CONFIGURATION MODELS
// ==========================================================

use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ReadmeConfig {
    pub path: String,
    pub file: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_project_name")]
    pub project_name: String,
    #[serde(default = "default_feature_name")]
    pub feature_name: String,
    #[serde(default = "default_package_name")]
    pub package_name: String,
    pub readme: Vec<ReadmeConfig>,
}
