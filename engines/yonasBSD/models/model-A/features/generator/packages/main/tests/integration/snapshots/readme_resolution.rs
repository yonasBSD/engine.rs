//! Snapshot test: readme template resolution for simple, non-templated paths.
//!
//! This verifies that readme templates are rendered into the expected locations.

use crate::helpers::*;
use engine_rs_lib::core::public::dsl::prelude::*;

#[test]
fn snapshot_readme_template_resolution() -> miette::Result<()> {
    let h = ScaffolderTestHarness::new();

    let cfg = ProjectsPhase::new()
        .add_project("demo")
        .next()
        .enable_feature("alpha")
        .next()
        .include_package("api")
        .next()
        .add_readme("readme/example.md.tpl", "engines/demo")
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

    // Snapshot the rendered README files
    h.snapshot_file("readme_demo", "engines/demo/README.md");

    Ok(())
}
