use engine_rs_lib::core::public::dsl::prelude::*;

fn main() {
    // Build a simple configuration using the staged DSL.
    let cfg = ProjectsPhase::new()
        .project("demo")
        .next()
        .feature("alpha")
        .next()
        .package("api")
        .readme("readme/example.md.tpl", "engines/demo")
        .finish()
        .build();

    // Print the resulting Config so users can see the structure.
    println!("{:#?}", cfg);
}
