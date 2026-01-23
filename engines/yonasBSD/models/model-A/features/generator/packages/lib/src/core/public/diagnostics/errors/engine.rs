use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("{0}")]
pub struct FullSource(pub String);

#[derive(Debug, Error, Diagnostic)]
pub enum EngineError {
    #[error("Invalid custom module path: `{path}`")]
    #[diagnostic(code("engine.invalid_path.empty_segment"))]
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
}

fn find_empty_segments(path: &str) -> Vec<SourceSpan> {
    let mut spans = Vec::new();
    let chars: Vec<(usize, char)> = path.char_indices().collect();

    for window in chars.windows(2) {
        let (idx, c1) = window[0];
        let (_, c2) = window[1];

        if c1 == '.' && c2 == '.' {
            spans.push(SourceSpan::new(idx.into(), 1usize.into()));
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
