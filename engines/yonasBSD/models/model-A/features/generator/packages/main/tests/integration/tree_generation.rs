//! Integration test: tree generation for custom modules.
//!
//! This refactored version still validates the generated tree structure,
//! but uses the test harness + config builder DSL for consistency.

use crate::helpers::*;
use engine_rs_lib::*;

#[test]
fn custom_modules_are_generated() {
    // 1. Create an isolated test environment
    let h = ScaffolderTestHarness::new();

    // 2. Build a config.toml with custom modules using the DSL
    let cfg = ConfigBuilder::new()
        .project("demo")
        .feature("alpha")
        .package("api")
        .custom_module("api.core", &["graphql", "grpc", "rest"]);

    // 3. Write config.toml and run the scaffolder
    h.write_config_builder(cfg);
    h.run();

    // 4. Assert the custom module tree exists
    let base = "engines/demo/models/model-A/features/alpha/packages/api/core/backends";

    for backend in ["graphql", "grpc", "rest"] {
        let dir = format!("{}/{}", base, backend);

        h.assert_exists(&dir);
        h.assert_exists(format!("{}/tests/mod.rs", dir));
        h.assert_exists(format!("{}/tests/unit/mod.rs", dir));
        h.assert_exists(format!("{}/tests/integration/mod.rs", dir));
        h.assert_exists(format!("{}/mod.rs", dir));
    }

    // 5. Snapshot the tree for regression coverage
    h.snapshot_tree("custom_modules_tree_generation");

    // 6. Sanity check: ensure the base path appears in the tree
    h.assert_tree_contains(base);
}
