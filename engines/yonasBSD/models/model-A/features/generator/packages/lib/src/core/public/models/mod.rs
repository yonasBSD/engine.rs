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
    #[serde(default = "default_projects")]
    pub projects: Vec<String>,
    #[serde(default = "default_features")]
    pub features: Vec<String>,
    #[serde(default = "default_packages")]
    pub packages: Vec<String>,
    pub readme: Vec<ReadmeConfig>,
}
