use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::ScaffolderError;

#[derive(Debug, Error, Diagnostic)]
#[error("{0}")]
pub struct FullSource(pub String);

#[derive(Debug, Error, Diagnostic)]
pub enum EngineError {
    /// Invalid custom module path.
    ///
    /// This error occurs when a custom module path contains one or more empty
    /// segments. Empty segments appear when two dots occur consecutively,
    /// like `api..core`.
    ///
    /// # Example
    ///
    /// ```text
    /// api..core
    /// ```
    #[error("Invalid custom module path: `{path}`")]
    #[diagnostic(code = "E0001")]
    InvalidPath {
        #[allow(unused)]
        path: String,

        #[allow(unused)]
        spans: Vec<SourceSpan>,

        #[help]
        #[allow(unused)]
        suggestion: Option<String>,

        #[source_code]
        #[allow(unused)]
        src: NamedSource<String>,

        #[diagnostic_source]
        #[allow(unused)]
        full: FullSource,
    },

    /// Wraps any scaffolder-level diagnostic (DirectoryMissing, ReadmeMissing,
    /// etc.)
    #[error(transparent)]
    #[diagnostic(transparent)]
    Scaffolder(#[from] ScaffolderError),
}

impl EngineError {
    pub fn invalid_path(path: impl Into<String>) -> Self {
        let path = path.into();
        let spans = find_empty_segments(&path);
        let suggestion = suggest_fixed_path(&path);

        EngineError::InvalidPath {
            src: NamedSource::new("engine", path.clone()),
            full: FullSource(path.clone()),
            path,
            spans,
            suggestion,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            EngineError::InvalidPath {
                ..
            } => "E0001",
            EngineError::Scaffolder(err) => err.code_str(),
        }
    }
}

fn find_empty_segments(path: &str) -> Vec<SourceSpan> {
    let mut spans = Vec::new();
    let chars: Vec<(usize, char)> = path.char_indices().collect();

    for window in chars.windows(2) {
        let (idx, c1) = window[0];
        let (_, c2) = window[1];

        if c1 == '.' && c2 == '.' {
            // zero-length span → rustc-style pointing label
            spans.push(SourceSpan::new(idx.into(), 0usize.into()));
        }
    }

    spans
}

fn suggest_fixed_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let cleaned: Vec<&str> = parts.into_iter().filter(|p| !p.is_empty()).collect();

    if cleaned.len() > 1 {
        Some(cleaned.join("."))
    } else {
        None
    }
}
