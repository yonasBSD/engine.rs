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
    "packages/cli",
    "packages/api",
    "packages/lib",
    "packages/testing",
]
resolver = "2""#;

pub const TPL_MEMBER_CARGO: &str = r#"[package]
name = "{{ package }}"
version = "0.1.0"
edition = "2024"

[lib]
name = "{{ package | replace("-", "_") | lower }}"
path = "src/lib.rs""#;

pub const TPL_MOD_EXPORT: &str = r"pub mod core;
pub mod enums;
pub mod macros;
pub mod traits;
pub mod utils;";

pub const TPL_MOD_TESTS: &str = "pub mod unit;\npub mod integration;\n";

pub const EXTRA_TOP_LEVEL_DIRS: &[&str] = &[
    "benches", "docs", "contrib", "scripts", "examples", "vendor",
];
