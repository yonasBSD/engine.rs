use miette::Result;

/// Run the `generate` subcommand.
///
/// This command generates the Engine.rs error index by walking the
/// `engine-rs-lib` crate, extracting all `EngineError` variants, validating
/// them, and rendering a Markdown document into the CLI assets directory.
pub fn run() -> Result<()> {
    // Delegate to the shared index generation logic in
    // `utils::generate_error_index`.
    crate::utils::generate_error_index()
}
