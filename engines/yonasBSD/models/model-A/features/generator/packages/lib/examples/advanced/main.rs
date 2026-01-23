use engine_rs_lib::core::public::dsl::prelude::*;

fn main() {
    // A full configuration demonstrating:
    // - multiple projects
    // - multiple features
    // - multiple packages
    // - custom modules with dotted paths
    // - extra folders
    // - readme templates
    // - final build into Config

    /*
    let cfg = ProjectsPhase::new()
        .project("core")
        .project("cli")
        .next()
        .feature("logging")
        .feature("tracing")
        .next()
        .package("api")
        .package("lib")
        .readme("readme/overview.md.tpl", "engines/{{ project }}/docs")
        .readme("readme/usage.md.tpl", "engines/{{ project }}/docs")
        .add_custom_module("api.core", &["graphql", "grpc", "rest"])
        .add_custom_module("lib.storage", &["s3", "local"])
        .extra_folder("scripts")
        .extra_folder("templates")
        .finish()
        .build();

    println!("{:#?}", cfg);
    */
}
