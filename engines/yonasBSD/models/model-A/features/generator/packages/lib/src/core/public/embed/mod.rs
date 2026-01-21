// ==========================================================
// EMBEDDED ASSETS & DEFAULTS
// ==========================================================

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

pub const DEFAULT_README_TPL: &str = "# {{ project_name }}\nGenerated via Scaffolder.\n";
pub const TPL_CARGO: &str = r#"[workspace]
members = [
    "{{ feature_name }}/packages/cli",
    "{{ feature_name }}/packages/api",
    "{{ feature_name }}/packages/lib",
    "{{ feature_name }}/packages/testing",
    "{{ feature_name }}/packages/{{ package_name }}",
]
resolver = "2""#;

pub const TPL_MOD_EXPORT: &str = r#"pub mod core;
pub mod enums;
pub mod macros;
pub mod traits;
pub mod utils;

// Core internal exports
pub mod backends;
pub mod frontends;
"#;

pub const TPL_MOD_TESTS: &str = "pub mod unit;\npub mod integration;\n";

pub const EXTRA_TOP_LEVEL_DIRS: &[&str] = &[
    "benches", "docs", "contrib", "scripts", "examples", "vendor",
];
