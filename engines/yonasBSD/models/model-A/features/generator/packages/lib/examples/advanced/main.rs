use engine_rs_lib::core::public::dsl::prelude::*;

fn main() {
    let cfg = ProjectsPhase::new()
        // --- Projects ---
        .project("core")
        .project("cli")
        .next()
        // --- Features ---
        .feature("logging")
        .feature("tracing")
        // Extra folders belong to FeaturesPhase
        .extra_folder("scripts")
        .extra_folder("templates")
        .next()
        // --- Packages ---
        .package("api")
        .package("lib")
        .next()
        // --- Custom Modules ---
        .add_custom_module("api.core", &["graphql", "grpc", "rest"])
        .expect("valid custom module path")
        .add_custom_module("lib.storage", &["s3", "local"])
        .expect("valid custom module path")
        .next()
        // --- README Templates ---
        .readme("readme/overview.md.tpl", "engines/{{ project }}/docs")
        .readme("readme/usage.md.tpl", "engines/{{ project }}/docs")
        .finish()
        .build();

    println!("{:#?}", cfg);
}
