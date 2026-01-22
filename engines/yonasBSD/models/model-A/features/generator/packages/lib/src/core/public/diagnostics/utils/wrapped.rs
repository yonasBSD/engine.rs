use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct WrappedDiagnostic(pub Box<dyn Diagnostic + Send + Sync>);

pub fn wrap<D>(diag: D) -> WrappedDiagnostic
where
    D: Diagnostic + Send + Sync + 'static,
{
    WrappedDiagnostic(Box::new(diag))
}
