//! Snapshot test: readme template resolution for simple, non-templated paths.
//!
//! This verifies that readme templates are rendered into the expected
//! locations.

use engine_rs_lib::core::public::dsl::prelude::*;

use crate::helpers::*;

#[test]
fn snapshot_readme_template_resolution() -> miette::Result<()> {
    let h = ScaffolderTestHarness::new();

    let cfg = ProjectsPhase::new()
        .project("demo")
        .next()
        .feature("alpha")
        .next()
        .package("api")
        .next()
        .readme("readme/example.md.tpl", "engines/demo")
        .readme(
            "readme/api.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/api",
        )
        .readme(
            "readme/lib.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/lib",
        )
        .finish()
        .build();

    h.write_config(&cfg);
    h.run();

    // README files must exist
    h.assert_all_readmes_exist(&cfg)?;

    // Snapshot the rendered README files
    h.snapshot_file("readme_demo", "engines/demo/README.md");

    Ok(())
}
