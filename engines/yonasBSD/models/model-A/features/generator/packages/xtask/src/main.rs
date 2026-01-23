mod commands;
mod utils;

use crate::enums::*;
use clap::Parser;
use commands::*;

/// Process-wide entrypoint for the `xtask` binary.
///
/// This binary is intended to be invoked as `cargo xtask <subcommand>` and
/// provides developer-focused workflows for maintaining Engine.rs diagnostics.
fn main() {
    // Disable Rust backtraces for this utility to keep output focused on
    // Miette diagnostics and human-readable error messages.
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "0");
    }

    // Route into the real main logic and surface any diagnostics in a simple
    // debug format. Non-zero exit codes signal failure to CI and scripts.
    if let Err(err) = real_main() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}

/// Real main logic for the `xtask` binary.
///
/// This function parses CLI arguments, dispatches to the appropriate
/// subcommand handler, and propagates any Miette diagnostics upward.
fn real_main() -> miette::Result<()> {
    // Parse CLI arguments using Clap's derive-based interface.
    let cli = commands::Cli::parse();

    // Dispatch to the selected subcommand. Each handler returns a `Result<()>`
    // so that Miette diagnostics can bubble up naturally.
    match cli.command {
        Commands::Generate {} => commands::generate::run()?,
        Commands::New { code, name } => commands::new::run(code, name)?,
        Commands::Check {} => commands::check::run()?,
    }

    Ok(())
}
