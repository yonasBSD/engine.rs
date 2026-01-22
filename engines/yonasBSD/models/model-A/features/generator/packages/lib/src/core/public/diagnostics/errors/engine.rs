use thiserror::Error;

use crate::core::public::diagnostics::wrapped::WrappedDiagnostic;
use crate::core::public::diagnostics::errors::ScaffolderError;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error(transparent)]
    Diagnostic(#[from] WrappedDiagnostic),

    #[error(transparent)]
    Scaffolder(#[from] ScaffolderError),
}
