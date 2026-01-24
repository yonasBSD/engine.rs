use crate::{Config, core::public::dsl::prelude::ProjectsPhase};

/// Re-exported function-style wrapper if you prefer calling it from code.
///
/// Example:
/// ```rust
/// use engine_rs_lib::*;
///
/// let cfg = config! {
///     projects { "core", "cli" }
///     features { "logging", "tracing" }
///     packages { "engine-rs", "engine-rs-lib" }
/// };
/// ```
pub fn config_example() -> Config {
    crate::config! {
        projects { "core", "cli" }
        features { "logging", "tracing" }
        packages { "engine-rs", "engine-rs-lib" }
    }
}

pub struct DslNode<T> {
    pub value: T,
    pub span: miette::SourceSpan,
}
