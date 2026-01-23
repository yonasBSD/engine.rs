use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Generate the error index
    Generate {},

    /// Create a new error code + variant + sidecar
    New {
        code: String,
        #[arg(default_value = "NewError")]
        name: String,
    },

    /// Validate all error codes + sidecars
    Check {},
}
