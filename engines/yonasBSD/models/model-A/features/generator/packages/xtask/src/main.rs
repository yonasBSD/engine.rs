mod commands;
mod utils;

use crate::enums::*;
use clap::Parser;
use commands::*;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "0");
    }

    if let Err(err) = real_main() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}

fn real_main() -> miette::Result<()> {
    let cli = commands::Cli::parse();

    match cli.command {
        Commands::Generate {} => commands::generate::run()?,
        Commands::New { code, name } => commands::new::run(code, name)?,
        Commands::Check {} => commands::check::run()?,
    }

    Ok(())
}
