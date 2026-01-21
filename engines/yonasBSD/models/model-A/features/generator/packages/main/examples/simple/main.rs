use engine_rs_lib::core::*;

fn main() {
    let cfg = ConfigBuilder::new()
        .project("demo")
        .feature("alpha")
        .package("api")
        .package("lib")
        .readme("readme/example.md.tpl", "engines/{{ project }}")
        .readme(
            "readme/api.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/api"
        )
        .readme(
            "readme/lib.md.tpl",
            "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/lib"
        )
        .custom_module("api.core", &["graphql", "grpc", "rest"]);

    println!("{}", cfg.build());
}
