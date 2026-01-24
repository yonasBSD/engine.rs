mod commands;
mod traits;
mod utils;

use commands::*;
use traits::*;
use utils::*;

use clap::Parser;
use engine_rs_lib::handlers::install_ariadne_hook;
use std::io;

//
// MAIN ENTRYPOINT
//

fn main() -> io::Result<()> {
    install_ariadne_hook();

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
        Commands::Explain {
            code,
        } => cmd_explain(code),
    }?;

    Ok(())
}
