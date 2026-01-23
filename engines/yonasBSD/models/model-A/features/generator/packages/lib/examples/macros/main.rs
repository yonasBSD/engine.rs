use engine_rs_lib::config;
use engine_rs_lib::core::public::dsl::macros::*;
use engine_rs_lib::projects_phase::ProjectsPhase;

fn main() {
    // Declarative macro-based configuration.
    // This mirrors the TOML-like structure users may prefer.

    let cfg = config! {
        projects { "core", "cli" }
        features { "logging", "tracing" }
        packages { "api", "lib" }
    };

    println!("{:#?}", cfg);
}
