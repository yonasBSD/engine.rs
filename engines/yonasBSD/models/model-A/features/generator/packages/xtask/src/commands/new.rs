use miette::Result;

/// Run the `new` subcommand.
///
/// This command scaffolds a new error in the `EngineError` enum and creates
/// a corresponding Markdown sidecar file. It enforces the error code format
/// and prevents collisions with existing codes.
pub fn run(code: String, name: String) -> Result<()> {
    // Delegate to the shared scaffolding logic in `utils::new_error`.
    crate::utils::new_error(&code, Some(&name))
}
