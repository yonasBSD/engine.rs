mod commands;
mod traits;
mod utils;

use commands::*;
use traits::*;
use utils::*;

use clap::Parser;
use std::io;

//
// MAIN ENTRYPOINT
//

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Validate {
            explain,
            quiet,
            json,
        } => cmd_validate(explain, quiet, json),
        Commands::Run {
            explain,
            quiet,
            json,
            debug,
        } => cmd_run(explain, quiet, json, debug),
    }
}
