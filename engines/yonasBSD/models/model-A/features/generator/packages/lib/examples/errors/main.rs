use engine_rs_lib::core::public::dsl::prelude::*;
use engine_rs_lib::{EngineError, handlers::install_ariadne_hook};

fn main() {
    // This example intentionally triggers an error to show off
    // your beautiful Miette + Ariadne diagnostics.
    color_backtrace::install();
    install_ariadne_hook();

    // Example mistake: invalid custom module path (empty segment)
    let result = (|| -> Result<_, EngineError> {
        ProjectsPhase::new()
            .project("demo")
            .next()
            .feature("alpha")
            .next()
            .add_custom_module("api..core..a..b..c", &["graphql"])? // <-- now errors
            .finish()
            .build();
        Ok(())
    })();

    match result {
        Ok(_) => println!("Unexpected success — error example didn't trigger"),
        Err(err) => {
            eprintln!("{}", miette::Report::new(err)); // Miette pretty-print
        }
    }
}
