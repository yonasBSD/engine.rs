mod traits;

use engine_rs_lib::{core::Config, traits::RealFS, traits::Scaffolder};
use traits::*;

use cliclack::{intro, note, outro, progress_bar, spinner};
use console::style;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let _ = intro(style(" Engine.rs Scaffolder ").on_cyan().black());

    // 1. Load Configuration
    let config_raw = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        r#"
projects = ["yonasBSD"]
features = ["feature-A"]
packages = ["package-A"]

[[readme]]
path = "engines/yonasBSD"
file = "readme/engines.md.tpl"
"#
        .to_string()
    });

    let config: Config = toml::from_str(&config_raw).expect("Invalid TOML config");

    // Build a readable list of projects for UI messages
    let project_list = config.projects.join(", ");

    // 2. Setup Progress & Scaffolder
    let pb = progress_bar(50);
    let lfs = LoggingFS::new(RealFS, &pb);
    let scaffolder = Scaffolder::new(lfs, PathBuf::from("."));

    // 3. Execution Phase
    pb.start(format!("Building structures for: {}", project_list));

    let manifest = scaffolder.run(config.clone())?;
    scaffolder.fs.clear_ui_lines();
    pb.stop("Generation complete.");

    // 4. Verification Phase
    let s = spinner();
    s.start("Executing BLAKE-3 Deep Verification...");

    match scaffolder.verify_integrity(manifest) {
        Ok(elapsed) => {
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
        Err(errors) => {
            s.stop("Integrity Compromised!");
            for err in errors {
                eprintln!("{} {}", style("✘").red(), err);
            }
            std::process::exit(1);
        }
    }

    Ok(())
}
