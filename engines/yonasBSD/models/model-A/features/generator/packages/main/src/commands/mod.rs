//
// CLI DEFINITIONS
//

mod enums;
mod explain;
mod init;
mod run;
mod validate;

pub use enums::*;
pub use explain::*;
pub use init::*;
pub use run::*;
pub use validate::*;

use clap::Parser;

#[derive(Parser)]
#[command(name = "engine-rs", version, about = "Engine.rs Scaffolder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
