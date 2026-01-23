pub mod check;
pub mod enums;
pub mod generate;
pub mod new;

use clap::Parser;
use enums::Commands;

#[derive(Parser)]
#[command(name = "xtask", version, about = "Engine.rs developer tasks")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
