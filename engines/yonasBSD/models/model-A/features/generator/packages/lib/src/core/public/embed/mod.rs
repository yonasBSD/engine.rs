// ==========================================================
// EMBEDDED ASSETS & DEFAULTS
// ==========================================================

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "templates/"]
pub struct Asset;

pub const DEFAULT_README_TPL: &str = "# {{ project_name }}\nGenerated via engine.rs.\n";

/// Workspace Cargo.toml template.
/// Context variables:
///   - `members`   : pre-rendered newline-joined member strings
///   - `workspace` : WorkspaceMetadata (version, edition, description, license,
///     repository, keywords, categories)
pub const TPL_CARGO: &str = r#"[workspace]
members = [
{{ members }}
]
resolver = "2"

[workspace.package]
version = "{{ workspace.version }}"
edition = "{{ workspace.edition }}"
description = "{{ workspace.description }}"
license = "{{ workspace.license }}"
repository = "{{ workspace.repository }}"
keywords = {{ workspace.keywords | tojson }}
categories = {{ workspace.categories | tojson }}"#;

/// Member Cargo.toml template.
/// All fields inherit from the workspace.
/// The `lib` package omits the `lib = { path = "../lib" }` self-dependency.
pub const TPL_MEMBER_CARGO: &str = r#"[package]
name = "{{ package }}"
version = { workspace = true }
edition = { workspace = true }
description = { workspace = true }
license = { workspace = true }
repository = { workspace = true }
keywords = { workspace = true }
categories = { workspace = true }

[lib]
name = "{{ package | replace("-", "_") | lower }}"
path = "src/lib.rs"

[dependencies]
{% if package != "lib" %}lib = { path = "../lib" }{% endif %}"#;

pub const TPL_MOD_EXPORT: &str = r"pub mod core;
pub mod enums;
pub mod macros;
pub mod traits;
pub mod utils;";

pub const TPL_MOD_TESTS: &str = "pub mod unit;\npub mod integration;\n";

pub const EXTRA_TOP_LEVEL_DIRS: &[&str] = &[
    "benches", "docs", "contrib", "scripts", "examples", "vendor",
];
