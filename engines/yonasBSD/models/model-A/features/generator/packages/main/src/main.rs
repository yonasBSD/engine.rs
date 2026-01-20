use cliclack::{intro, note, outro, progress_bar, spinner};
use console::style;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use engine_rs::{core::Config, traits::FileSystem, traits::RealFS, traits::Scaffolder};

// ==========================================================
// 1. UI DECORATOR (LoggingFS)
// ==========================================================
/// A decorator for any FileSystem that updates the Clack progress bar
struct LoggingFS<'a, F: FileSystem> {
    inner: F,
    pb: &'a cliclack::ProgressBar,
}

impl<'a, F: FileSystem> LoggingFS<'a, F> {
    fn new(inner: F, pb: &'a cliclack::ProgressBar) -> Self {
        Self { inner, pb }
    }

    fn log_ephemeral(&self, action: &str, path: &Path) {
        self.pb.inc(1);
        // ANSI escape codes to clear the line and print the update above the bar
        print!(
            "\x1B[1A\x1B[2K\r  {} {}: {:?}\n",
            style("⚡").yellow(),
            action,
            path
        );
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(5));
    }

    fn clear_ui_lines(&self) {
        print!("\x1B[1A\x1B[2K\r\x1B[1A\x1B[2K\r");
        let _ = io::stdout().flush();
    }
}

impl<'a, F: FileSystem> FileSystem for LoggingFS<'a, F> {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        self.log_ephemeral("mkdir", path);
        self.inner.create_dir_all(path)
    }
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        self.log_ephemeral("write", path);
        self.inner.write_file(path, content)
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.inner.read_to_string(path)
    }
}

// ==========================================================
// 2. MAIN RUNTIME
// ==========================================================
fn main() -> io::Result<()> {
    let _ = intro(style(" yonasBSD Engine Scaffolder ").on_cyan().black());

    // 1. Load Configuration
    let config_raw = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        r#"project_name = "yonasBSD"
[[readme]]
path = "engines/yonasBSD"
file = "readme/engines.md.tpl""#
            .to_string()
    });

    let config: Config = toml::from_str(&config_raw).expect("Invalid TOML config");

    // 2. Setup Progress & Scaffolder
    // We estimate ~50 files/dirs for the mirrored 12-level hierarchy
    let pb = progress_bar(50);
    let lfs = LoggingFS::new(RealFS, &pb);
    let scaffolder = Scaffolder::new(lfs, PathBuf::from("."));

    // 3. Execution Phase
    pb.start(format!("Building {} structure...", config.project_name));
    let manifest = scaffolder.run(config.clone())?;
    scaffolder.fs.clear_ui_lines();
    pb.stop("Generation complete.");

    // 4. Verification Phase
    let s = spinner();
    s.start("Executing BLAKE3 Deep Verification...");

    match scaffolder.verify_integrity(manifest) {
        Ok(elapsed) => {
            s.stop(format!("Integrity Verified in {}ms.", elapsed.as_millis()));

            let _ = note(
                "Next Steps",
                format!(
                    "Project {} generated. Run {} to build the engine.",
                    style(&config.project_name).cyan(),
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
