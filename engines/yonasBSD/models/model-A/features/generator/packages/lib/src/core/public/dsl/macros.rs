use crate::Config;
use crate::core::public::dsl::prelude::ProjectsPhase;

/// Declarative macro entry point for the config DSL.
///
/// This macro expands into the typed, staged DSL:
/// ProjectsPhase → FeaturesPhase → PackagesPhase → FinalPhase → Config
#[macro_export]
macro_rules! config {
    (
        projects { $($proj:expr),* $(,)? }
        features { $($feat:expr),* $(,)? }
        packages { $($pkg:expr),* $(,)? }
        $(,)?
    ) => {{
        ProjectsPhase::new()
            $(.project($proj))*
            .next()
            $(.feature($feat))*
            .next()
            $(.package($pkg))*
            .finish()
            .build()
    }};
}

/// Re-exported function-style wrapper if you prefer calling it from code.
///
/// Example:
/// ```rust
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
