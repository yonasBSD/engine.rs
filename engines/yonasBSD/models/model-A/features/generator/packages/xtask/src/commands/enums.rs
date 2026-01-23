use clap::Subcommand;

/// Top-level `xtask` subcommands.
///
/// These subcommands are intentionally narrow and map directly to focused
/// workflows for maintaining the Engine.rs error system.
#[derive(Subcommand)]
pub enum Commands {
    /// Generate the error index.
    ///
    /// Walks the `engine-rs-lib` crate, extracts all `EngineError` variants,
    /// validates them, and writes a rendered Markdown index to the CLI assets
    /// directory.
    Generate {},

    /// Create a new error code + variant + sidecar.
    ///
    /// This scaffolds a new `EngineError` variant and a corresponding Markdown
    /// sidecar file. It enforces the `E0001`-style format and prevents
    /// collisions with existing codes.
    New {
        /// The new error code to create (e.g. `E0001`).
        code: String,

        /// The Rust enum variant name to insert into `EngineError`.
        ///
        /// Defaults to `NewError` if not provided.
        #[arg(default_value = "NewError")]
        name: String,
    },

    /// Validate all error codes + sidecars.
    ///
    /// This runs the same validation logic used by `generate`, but without
    /// writing the index file. It is ideal for CI or quick local checks.
    Check {},
}
