use std::{io, path::PathBuf};

use cliclack::{
    intro,
    log::{error, step, warning},
    note, outro, progress_bar, spinner,
};
use console::style;
use engine_rs_lib::traits::{RealFS, Scaffolder};

use crate::{
    LoggingFS, is_quiet, load_config, print_explain_rules, print_json_integrity_errors,
    print_json_ok, print_json_validation_errors,
};

//
// RUN COMMAND
//

pub fn cmd_run(explain: bool, quiet: bool, json: bool, debug: bool) -> io::Result<()> {
    let config = load_config();

    if explain && !is_quiet(quiet, json) {
        print_explain_rules();
    }

    // VALIDATION FIRST
    if let Err(errors) = config.validate() {
        if json {
            print_json_validation_errors(&errors);
        } else {
            let _ = error("Validation Errors:");
            for err in errors {
                let _ = warning(format!("{err}"));
            }
        }
        std::process::exit(1);
    }

    if !is_quiet(quiet, json) {
        let _ = intro(style(" Engine.rs Scaffolder ").on_cyan().black());
    }

    // PROGRESS BAR + SCAFFOLDER
    let project_list = config.projects.join(", ");
    let pb = progress_bar(50);
    let lfs = LoggingFS::new(RealFS, &pb);
    let scaffolder = Scaffolder::new(lfs, PathBuf::from("."));

    if !is_quiet(quiet, json) {
        pb.start(format!("Building structures for: {project_list}"));
    }

    let manifest = scaffolder.run(&config)?;

    scaffolder.fs.clear_ui_lines();

    if !is_quiet(quiet, json) {
        pb.stop("Generation complete.");
    }

    // DEBUG MODE
    if debug && !json {
        let _ = note("Debug Output", "Internal scaffolder paths:");
        for (path, hash) in &manifest {
            let _ = step(format!("{}  ({})", path.display(), hash));
        }
    }

    // VERIFICATION
    let s = spinner();
    if !is_quiet(quiet, json) {
        s.start("Executing BLAKE-3 Deep Verification...");
    }

    match scaffolder.verify_integrity(manifest) {
        Ok(elapsed) => {
            if json {
                print_json_ok(&config);
            } else if !quiet {
                s.stop(format!("Integrity Verified in {}ms.", elapsed.as_millis()));

                let _ = note(
                    "Next Steps",
                    format!(
                        "Generated engines for: {}.\nRun {} to build the engine workspace.",
                        style(&project_list).cyan(),
                        style("just build").yellow()
                    ),
                );

                let _ = outro(style(" Build Success ").black().on_green());
            }
        },
        Err(errors) => {
            if json {
                print_json_integrity_errors(&errors);
            } else {
                s.stop("Integrity Compromised!");
                let _ = error("Integrity check failed:");
                for err in errors {
                    let _ = warning(err.clone());
                }
            }
            std::process::exit(1);
        },
    }

    Ok(())
}
