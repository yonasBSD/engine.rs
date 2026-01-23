pub mod check;
pub mod enums;
pub mod generate;
pub mod new;

use clap::Parser;
use enums::Commands;

/// Top-level CLI entrypoint for `cargo xtask`.
///
/// This struct defines the `xtask` interface used by Engine.rs developers to
/// maintain error codes, sidecars, and the generated error index.
#[derive(Parser)]
#[command(name = "xtask", version, about = "Engine.rs developer tasks")]
pub struct Cli {
    /// The subcommand to execute.
    ///
    /// Each subcommand corresponds to a focused maintenance workflow.
    #[command(subcommand)]
    pub command: Commands,
}
