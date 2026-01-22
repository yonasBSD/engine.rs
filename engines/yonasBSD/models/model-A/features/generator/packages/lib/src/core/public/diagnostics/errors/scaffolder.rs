use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ScaffolderError {
    #[error("Template resolution failed: Directory was not created")]
    #[diagnostic(
        code(scaffolder::path_missing),
        help("The engine failed to resolve placeholders or create this directory: {path}")
    )]
    DirectoryMissing { path: String },

    #[error("README file missing in generated directory")]
    #[diagnostic(
        code(scaffolder::readme_missing),
        help("The directory exists, but README.md is missing at: {path}")
    )]
    ReadmeMissing { path: String },
}
