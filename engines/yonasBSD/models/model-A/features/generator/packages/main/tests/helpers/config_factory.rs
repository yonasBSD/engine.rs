use engine_rs_lib::core::public::dsl::prelude::*;

pub fn demo_api_alpha_config() -> Config {
    ProjectsPhase::new()
        .add_project("demo")
        .next()
        .enable_feature("alpha")
        .next()
        .include_package("api")
        .next()
        .add_custom_module("api.core", &["graphql", "grpc", "rest"])
        .build()
}

#[test]
fn tree_generation_matches_snapshot() {
    use crate::helpers::config_factory::demo_api_alpha_config;

    let cfg = demo_api_alpha_config();
    // existing assertions / snapshot harness
}
