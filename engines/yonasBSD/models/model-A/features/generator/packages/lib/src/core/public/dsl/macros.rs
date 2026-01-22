use crate::Config;
use crate::dsl::projects::ProjectsPhase;

/// Macro entry point for a declarative config DSL.
///
/// This is a stub that currently just forwards to a typed builder.
/// You can evolve the syntax later without breaking the typed phases.
#[macro_export]
macro_rules! config {
    (
        projects { $($proj:expr),* $(,)? }
        features { $($feat:expr),* $(,)? }
        packages { $($pkg:expr),* $(,)? }
    ) => {{
        let builder = ProjectsPhase::new()
            $(.add_project($proj))*
            .next()
            $(.enable_feature($feat))*
            .next()
            $(.include_package($pkg))*
            .next();

        builder.build()
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
