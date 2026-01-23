//
// CLI DEFINITIONS
//

mod enums;
mod init;
mod run;
mod validate;
mod explain;

pub use enums::*;
pub use init::*;
pub use run::*;
pub use validate::*;
pub use explain::*;

use clap::Parser;

#[derive(Parser)]
#[command(name = "engine-rs", version, about = "Engine.rs Scaffolder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
