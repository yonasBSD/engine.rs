mod commands;
mod traits;
mod utils;

use std::io;

use clap::Parser;
use commands::{Cli, Commands, cmd_explain, cmd_init, cmd_run, cmd_validate};
use engine_rs_lib::handlers::install_ariadne_hook;
use traits::LoggingFS;
use utils::{
    is_quiet, load_config, print_explain_rules, print_json_integrity_errors, print_json_ok,
    print_json_validation_errors,
};

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
