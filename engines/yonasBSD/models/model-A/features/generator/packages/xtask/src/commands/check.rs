use miette::Result;

/// Run the `check` subcommand.
///
/// This command scans the `engine-rs-lib` crate for all `EngineError` variants,
/// validates their documentation, sidecars, numeric sequencing, and enum
/// ordering, and emits structured diagnostics on failure.
///
/// It is intended to be used in CI and locally to enforce error-discipline
/// across the codebase.
pub fn run() -> Result<()> {
    // Delegate to the shared validation logic in `utils::check_errors`.
    crate::utils::check_errors()
}
