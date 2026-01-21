//! Snapshot test: readme template resolution with templated paths.
//!
//! This verifies that `{{ project }}`, `{{ model }}`, and `{{ feature }}`
//! are correctly expanded in readme target paths.

use crate::helpers::*;
use crate::engine;
use engine_rs_lib::*;

#[test]
fn snapshot_readme_templated_paths() {
    let h = ScaffolderTestHarness::new();

    let cfg = engine!(
        projects = ["demo"],
        features = ["alpha"],
        packages = ["api", "lib"],
        readme("readme/example.md.tpl", "engines/{{ project }}"),
        readme(
            "readme/api.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/api"
        ),
        readme(
            "readme/lib.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/lib"
        )
    );

    h.write_config_builder(cfg);
    h.run();

    // Snapshot the full tree for context
    h.snapshot_tree("tree");

    // Snapshot each rendered README at its intended path
    h.snapshot_file("readme_global", "engines/demo/README.md");
    h.snapshot_file(
        "readme_api",
        "engines/demo/models/model-A/features/alpha/packages/api/README.md",
    );
    h.snapshot_file(
        "readme_lib",
        "engines/demo/models/model-A/features/alpha/packages/lib/README.md",
    );
}
