use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::core::public::diagnostics::ScaffolderError;
use crate::core::public::dsl::default_span;
use crate::wrapped::WrappedDiagnostic;

#[derive(Debug, Error, Diagnostic)]
pub enum EngineError {
    #[error(transparent)]
    Diagnostic(#[from] WrappedDiagnostic),

    #[error(transparent)]
    Scaffolder(#[from] ScaffolderError),

    #[error("Invalid custom module path: `{path}`")]
    #[diagnostic(
        code(engine::invalid_path),
        help("Module paths cannot contain empty segments.")
    )]
    InvalidPath {
        path: String,

        #[label("empty segment here")]
        span: SourceSpan,

        #[source_code]
        src: NamedSource<String>,

        #[diagnostic_source]
        full: FullSource,
    },
}

#[derive(Debug, Error, Diagnostic)]
#[error("{0}")]
pub struct FullSource(pub String);

impl EngineError {
    pub fn invalid_path(path: impl Into<String>) -> Self {
        let path = path.into();

        let mut span = default_span();

        let chars: Vec<(usize, char)> = path.char_indices().collect();
        for window in chars.windows(2) {
            let (idx, c1) = window[0];
            let (_, c2) = window[1];
            if c1 == '.' && c2 == '.' {
                span = SourceSpan::new(idx.into(), 1usize.into());
                break;
            }
        }

        EngineError::InvalidPath {
            src: NamedSource::new("engine", path.clone()),
            full: FullSource(path.clone()),
            path,
            span,
        }
    }
}
