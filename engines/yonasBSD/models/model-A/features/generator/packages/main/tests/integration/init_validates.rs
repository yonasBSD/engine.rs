//! Integration test: `engine-rs init` produces a valid config.toml,
//! and `engine-rs validate` accepts it.
//!
//! This test uses the harness to run the CLI in an isolated environment.

use engine_rs_lib::core::Config;
use std::fs;

use crate::helpers::HarnessExtensions;
use crate::helpers::ScaffolderTestHarness;

#[test]
fn init_produces_valid_config() {
    // Create an isolated test environment
    let h = ScaffolderTestHarness::new();

    println!("Running init in {}", h.path().display());

    // 1. Run `engine-rs init`
    // This should generate a default config.toml in the test root
    h.run_init();

    // 2. Ensure config.toml exists
    let config_path = h.path().join("config.toml");
    assert!(
        config_path.exists(),
        "config.toml missing after `engine-rs init`"
    );

    // 3. Parse it as TOML into a Config struct
    // This ensures the generated file is syntactically valid TOML
    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: Result<Config, _> = toml::from_str(&content);
    assert!(parsed.is_ok(), "config.toml is invalid TOML");

    // 4. Run `engine-rs validate`
    // This ensures the generated config passes your validation rules
    h.run_validate();
}
