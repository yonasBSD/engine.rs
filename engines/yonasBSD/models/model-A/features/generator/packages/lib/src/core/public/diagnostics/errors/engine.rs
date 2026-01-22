use thiserror::Error;
use miette::Diagnostic;

use crate::core::public::diagnostics::errors::ScaffolderError;
use crate::core::public::diagnostics::wrapped::WrappedDiagnostic;

#[derive(Debug, Error, Diagnostic)]
pub enum EngineError {
    #[error(transparent)]
    Diagnostic(#[from] WrappedDiagnostic),

    #[error(transparent)]
    Scaffolder(#[from] ScaffolderError),
}

impl From<ScaffolderError> for Result<(), EngineError> {
    fn from(err: ScaffolderError) -> Self {
        Err(err.into())
    }
}
