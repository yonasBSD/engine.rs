//! Snapshot test: custom module tree structure and backends mod.rs content.
//!
//! Uses the harness + config builder DSL + snapshot extensions.

use crate::helpers::*;
use engine_rs_lib::*;

#[test]
fn snapshot_custom_module_tree() {
    let h = ScaffolderTestHarness::new();

    let cfg = ConfigBuilder::new()
        .project("demo")
        .feature("alpha")
        .package("api")
        .custom_module("api.core", &["graphql", "grpc", "rest"]);

    h.write_config_builder(cfg);
    h.run();

    // Snapshot the directory tree
    h.snapshot_tree("custom_module_tree");

    // Snapshot a specific generated file
    h.snapshot_file(
        "backends_mod_rs",
        "engines/demo/models/model-A/features/alpha/packages/api/core/backends/mod.rs",
    );
}
