pub mod tests;
pub mod sync;

use crate::utils::*;

use rust_embed::RustEmbed;
use serde::Deserialize;

// ==========================================================
// 1. EMBEDDED ASSETS & DEFAULTS
// ==========================================================
#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

pub const DEFAULT_README_TPL: &str = "# {{ project_name }}\nGenerated via Scaffolder.\n";
pub const TPL_CARGO: &str = r#"[workspace]
members = ["{{ feature_name }}/packages/traits", "{{ feature_name }}/packages/{{ package_name }}"]
resolver = "2""#;

pub const TPL_MOD_EXPORT: &str = r#"pub mod core;
pub mod enums;
pub mod traits;
pub mod utils;

// Core internal exports
pub mod backends;
pub mod frontends;
"#;

pub const TPL_MOD_TESTS: &str = "pub mod unit;\npub mod integration;\n";

// ==========================================================
// 2. CONFIGURATION MODELS
// ==========================================================
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
