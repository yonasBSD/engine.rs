use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ScaffolderError {
    #[error("Template resolution failed: Directory was not created")]
    #[diagnostic(
        code = "S0001",
        help("The engine failed to resolve placeholders or create this directory: {path}")
    )]
    DirectoryMissing { path: String },

    #[error("README file missing in generated directory")]
    #[diagnostic(
        code = "S0002",
        help("The directory exists, but README.md is missing at: {path}")
    )]
    ReadmeMissing { path: String },
}

impl ScaffolderError {
    pub fn code_str(&self) -> &'static str {
        match self {
            ScaffolderError::DirectoryMissing {
                ..
            } => "S0001",
            ScaffolderError::ReadmeMissing {
                ..
            } => "S0002",
        }
    }
}
