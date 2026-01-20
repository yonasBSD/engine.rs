//
// CLI DEFINITIONS
//

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "engine-rs", version, about = "Engine.rs Scaffolder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate config.toml without generating anything
    Validate {
        /// Explain validation rules
        #[arg(long)]
        explain: bool,

        /// Quiet mode (no output unless errors)
        #[arg(long)]
        quiet: bool,

        /// Output validation result as JSON (for CI)
        #[arg(long)]
        json: bool,
    },

    /// Run the scaffolder (validates first)
    Run {
        /// Explain validation rules before running
        #[arg(long)]
        explain: bool,

        /// Quiet mode (no output unless errors)
        #[arg(long)]
        quiet: bool,

        /// Output results as JSON (for CI)
        #[arg(long)]
        json: bool,

        /// Debug mode: print internal scaffolder paths
        #[arg(long)]
        debug: bool,
    },

    /// Generate a default config.toml
    Init,
}
