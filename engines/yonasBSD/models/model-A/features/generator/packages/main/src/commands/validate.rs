use std::io;

use cliclack::{
    log::{error, success, warning},
    outro,
};
use console::style;

use crate::{
    is_quiet, load_config, print_explain_rules, print_json_ok, print_json_validation_errors,
};

//
// VALIDATE COMMAND
//

pub fn cmd_validate(explain: bool, quiet: bool, json: bool) -> io::Result<()> {
    let config = load_config();

    if explain && !is_quiet(quiet, json) {
        print_explain_rules();
    }

    match config.validate() {
        Ok(()) => {
            if json {
                print_json_ok(&config);
            } else if !quiet {
                let _ = success("Validation Passed");
                let _ = outro(style(" Validation Complete ").black().on_green());
            }
        },
        Err(errors) => {
            if json {
                print_json_validation_errors(&errors);
            } else {
                let _ = error("Validation Errors:");
                for err in errors {
                    let _ = warning(format!("{err}"));
                }
            }
            std::process::exit(1);
        },
    }

    Ok(())
}
