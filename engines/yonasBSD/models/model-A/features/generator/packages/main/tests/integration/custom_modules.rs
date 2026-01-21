//! Integration test: custom modules are generated correctly.
//!
//! This refactored version uses the full test helper framework:
//! - ScaffolderTestHarness for an isolated filesystem
//! - HarnessExtensions for running the CLI and writing config
//! - ConfigBuilder + ConfigBuilderDsl for type-safe config construction
//! - TreeAssertionDsl for tree-based assertions
//! - HarnessSnapshotExtensions for snapshotting the tree

use crate::helpers::*;
use engine_rs_lib::*;

#[test]
fn custom_modules_are_generated() {
    // 1. Create an isolated test environment
    let h = ScaffolderTestHarness::new();

    // 2. Build a config with custom modules using the fluent DSL
    let cfg = ConfigBuilder::new()
        .project("demo")
        .feature("alpha")
        .package("api")
        // This matches the original TOML:
        // [custom_modules.api.core]
        // backends = ["graphql", "grpc", "rest"]
        .custom_module("api.core", &["graphql", "grpc", "rest"]);

    // 3. Write config.toml and run the scaffolder
    h.write_config_builder(cfg);
    h.run();

    // 4. Assert the custom module tree exists at the intended path
    // engines/demo/models/model-A/features/alpha/packages/api/core/backends/<backend>/
    for backend in ["graphql", "grpc", "rest"] {
        let base = format!(
            "engines/demo/models/model-A/features/alpha/packages/api/core/backends/{}",
            backend
        );

        h.assert_exists(&base);
        h.assert_exists(format!("{}/tests/mod.rs", base));
        h.assert_exists(format!("{}/tests/unit/mod.rs", base));
        h.assert_exists(format!("{}/tests/integration/mod.rs", base));
        h.assert_exists(format!("{}/mod.rs", base));
    }

    // 5. Snapshot the tree for structural regression coverage
    h.snapshot_tree("custom_modules_tree");

    // 6. Sanity check: the tree should mention the backends directory
    h.assert_tree_contains("engines/demo/models/model-A/features/alpha/packages/api/core/backends");
}
