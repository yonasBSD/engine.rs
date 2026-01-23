//
// COMMAND IMPLEMENTATIONS
//

mod assets;
pub use assets::*;

use engine_rs_lib::core::{Config, ConfigError};

use cliclack::{
    log::{error, step},
    note,
};
use std::fs;

//
// LOAD CONFIG
//

pub fn load_config() -> Config {
    let raw = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        let _ = error("config.toml not found. Run `engine-rs init` first.");
        std::process::exit(1);
    });

    toml::from_str(&raw).unwrap_or_else(|_| {
        let _ = error("Invalid TOML in config.toml");
        std::process::exit(1);
    })
}

//
// EXPLAIN MODE
//

pub fn print_explain_rules() {
    let _ = note(
        "Validation Rules Explained",
        "A guided breakdown of all constraints:",
    );

    let _ = step("Lists must not be empty");
    let _ = step("No duplicates allowed");
    let _ = step("Names must match ^[a-zA-Z0-9_-]+$");
    let _ = step("Reserved names are forbidden");
    let _ = step("No two combinations may generate the same filesystem path");
}

pub const fn is_quiet(quiet: bool, json: bool) -> bool {
    quiet || json
}

pub fn print_json_ok(config: &Config) {
    let out = serde_json::json!({
        "status": "ok",
        "projects": config.projects,
        "features": config.features,
        "packages": config.packages,
    });
    println!("{out}");
}

pub fn print_json_validation_errors(errors: &[ConfigError]) {
    let out = serde_json::json!({
        "status": "error",
        "errors": errors.iter().map(std::string::ToString::to_string).collect::<Vec<_>>(),
    });
    println!("{out}");
}

pub fn print_json_integrity_errors(errors: &[String]) {
    let out = serde_json::json!({
        "status": "error",
        "errors": errors,
    });
    println!("{out}");
}

pub mod ui {
    pub fn success(msg: &str) {
        println!("\n\x1b[32m✔\x1b[0m {msg}");
    }

    pub fn error(msg: &str) {
        // Red "✘" followed by reset
        eprintln!("\n\x1b[1;31m✘\x1b[0m {msg}");
    }

    #[allow(dead_code)]
    pub fn warn(msg: &str) {
        eprintln!("\n\x1b[33m⚠\x1b[0m {msg}");
    }
}
