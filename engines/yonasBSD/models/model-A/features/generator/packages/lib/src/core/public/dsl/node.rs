use miette::SourceSpan;

#[derive(Debug, Clone)]
pub struct DslNode<T> {
    pub value: T,
    pub span: SourceSpan,
}

impl<T> DslNode<T> {
    pub const fn new(value: T, span: SourceSpan) -> Self {
        Self {
            value,
            span,
        }
    }
}

#[must_use]
pub fn default_span() -> SourceSpan {
    SourceSpan::new(0usize.into(), 0usize)
}
