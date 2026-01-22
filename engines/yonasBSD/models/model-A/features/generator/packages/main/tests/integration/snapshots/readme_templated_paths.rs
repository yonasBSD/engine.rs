//! Snapshot test: readme template resolution with templated paths.
//!
//! This verifies that `{{ project }}`, `{{ model }}`, and `{{ feature }}`
//! are correctly expanded in readme target paths.

use crate::helpers::*;
use engine_rs_lib::core::public::dsl::prelude::*;

#[test]
fn snapshot_readme_templated_paths() -> miette::Result<()> {
    let h = ScaffolderTestHarness::new();

    let cfg = ProjectsPhase::new()
        .add_project("demo")
        .next()
        .enable_feature("alpha")
        .next()
        .include_package("api")
        .include_package("lib")
        .next()
        .add_readme("readme/example.md.tpl", "engines/{{ project }}")
        .add_readme(
            "readme/api.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/api",
        )
        .add_readme(
            "readme/lib.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/lib",
        )
        .build();

    h.write_config(&cfg);
    h.run();

    // README files must exist
    h.assert_all_readmes_exist(&cfg)?;

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

    Ok(())
}
