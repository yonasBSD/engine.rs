use crate::utils::*;

use std::fs;
use std::io;

//
// INIT COMMAND
//

pub fn cmd_init() -> io::Result<()> {
    if std::path::Path::new("config.toml").exists() {
        let _ = error_msg("config.toml already exists.");
        std::process::exit(1);
    }

    let default = r#"
projects = ["example-project"]
features = ["feature-A"]
packages = ["package-A"]

[[readme]]
path = "engines/example-project"
file = "readme/example.md.tpl"
"#;

    fs::write("config.toml", default.trim_start())?;

    success_msg("Created default config.toml");
    Ok(())
}
