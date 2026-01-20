//
// COMMAND IMPLEMENTATIONS
//

use crate::*;
use engine_rs_lib::{
    core::{Config, ConfigError},
    traits::RealFS,
    traits::Scaffolder,
};

use cliclack::{
    intro,
    log::{error, step, success, warning},
    note, outro, progress_bar, spinner,
};
use console::style;
use std::fs;
use std::io;
use std::path::PathBuf;

//
// LOAD CONFIG
//

pub fn load_config() -> Config {
    let raw = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(_) => {
            let _ = error("config.toml not found. Run `engine-rs init` first.");
            std::process::exit(1);
        }
    };

    match toml::from_str(&raw) {
        Ok(cfg) => cfg,
        Err(_) => {
            let _ = error("Invalid TOML in config.toml");
            std::process::exit(1);
        }
    }
}

//
// INIT COMMAND
//

pub fn cmd_init() -> io::Result<()> {
    if std::path::Path::new("config.toml").exists() {
        let _ = error("config.toml already exists.");
        std::process::exit(1);
    }

    let default = r#"
projects = ["example-project"]
features = ["feature-A"]
packages = ["package-A"]

[[readme]]
path = "engines/example-project"
file = "readme/example.md.tpl"
"#;

    fs::write("config.toml", default.trim_start())?;

    let _ = success("Created default config.toml");
    Ok(())
}

//
// VALIDATE COMMAND
//

pub fn cmd_validate(explain: bool, quiet: bool, json: bool) -> io::Result<()> {
    let config = load_config();

    if explain && !is_quiet(quiet, json) {
        print_explain_rules();
    }

    match config.validate() {
        Ok(_) => {
            if json {
                print_json_ok(&config);
            } else if !quiet {
                let _ = success("Validation Passed");
                let _ = outro(style(" Validation Complete ").black().on_green());
            }
        }
        Err(errors) => {
            if json {
                print_json_validation_errors(&errors);
            } else {
                let _ = error("Validation Errors:");
                for err in errors {
                    let _ = warning(format!("{}", err));
                }
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

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
                let _ = warning(format!("{}", err));
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
        pb.start(format!("Building structures for: {}", project_list));
    }

    let manifest = scaffolder.run(config.clone())?;

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
        }
        Err(errors) => {
            if json {
                print_json_integrity_errors(&errors);
            } else {
                s.stop("Integrity Compromised!");
                let _ = error("Integrity check failed:");
                for err in errors {
                    let _ = warning(format!("{}", err));
                }
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

//
// EXPLAIN MODE
//

pub fn print_explain_rules() {
    let _ = note(
        "Validation Rules Explained",
        "A guided breakdown of all constraints:",
    );

    let _ = step("Lists must not be empty");
    let _ = step("No duplicates allowed");
    let _ = step("Names must match ^[a-zA-Z0-9_-]+$");
    let _ = step("Reserved names are forbidden");
    let _ = step("No two combinations may generate the same filesystem path");
}

pub fn is_quiet(quiet: bool, json: bool) -> bool {
    quiet || json
}

pub fn print_json_ok(config: &Config) {
    let out = serde_json::json!({
        "status": "ok",
        "projects": config.projects,
        "features": config.features,
        "packages": config.packages,
    });
    println!("{}", out);
}

pub fn print_json_validation_errors(errors: &[ConfigError]) {
    let out = serde_json::json!({
        "status": "error",
        "errors": errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
    });
    println!("{}", out);
}

pub fn print_json_integrity_errors(errors: &[String]) {
    let out = serde_json::json!({
        "status": "error",
        "errors": errors,
    });
    println!("{}", out);
}
