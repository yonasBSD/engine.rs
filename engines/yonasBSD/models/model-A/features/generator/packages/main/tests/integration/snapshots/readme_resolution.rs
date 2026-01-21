//! Snapshot test: readme template resolution for simple, non-templated paths.
//!
//! This verifies that readme templates are rendered into the expected locations.

use crate::helpers::{HarnessExtensions, HarnessSnapshotExtensions, ScaffolderTestHarness};

#[test]
fn snapshot_readme_template_resolution() {
    let h = ScaffolderTestHarness::new();

    h.write_config(
        r#"
        projects = ["demo"]
        features = ["alpha"]
        packages = ["api"]

        [[readme]]
        path = "engines/demo"
        file = "readme/example.md.tpl"

        [[readme]]
        path = "engines/demo/models"
        file = "readme/benches.md.tpl"
    "#,
    );

    h.run();

    // Snapshot the rendered README files
    h.snapshot_file("readme_demo", "engines/demo/README.md");
    h.snapshot_file("readme_models", "engines/demo/models/README.md");
}
