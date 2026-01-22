mod commands;
mod traits;
mod utils;

use commands::{Cli, Commands, cmd_init, cmd_run, cmd_validate};
use traits::LoggingFS;
use utils::{
    is_quiet, load_config, print_explain_rules, print_json_integrity_errors, print_json_ok,
    print_json_validation_errors,
};

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
