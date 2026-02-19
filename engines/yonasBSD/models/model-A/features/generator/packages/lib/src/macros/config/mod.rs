/// Declarative macro entry point for the config DSL.
///
/// This macro expands into the typed, staged DSL:
/// `ProjectsPhase` → `FeaturesPhase` → `PackagesPhase` → `FinalPhase` → Config
///
/// # Basic usage
/// ```rust
/// use engine_rs_lib::*;
///
/// let cfg = config! {
///     projects { "core", "cli" }
///     features { "logging", "tracing" }
///     packages { "api", "lib" }
/// };
/// ```
///
/// # With workspace metadata
/// ```rust
/// use engine_rs_lib::*;
///
/// let cfg = config! {
///     workspace {
///         description: "My project",
///         repository: "https://github.com/user/repo",
///     }
///     projects { "core", "cli" }
///     features { "logging" }
///     packages { "api", "lib" }
/// };
/// ```
#[macro_export]
macro_rules! config {
    // Arm with workspace metadata
    (
        workspace { $($field:ident: $val:expr),* $(,)? }
        projects { $($proj:expr),* $(,)? }
        features { $($feat:expr),* $(,)? }
        packages { $($pkg:expr),* $(,)? }
        $(,)?
    ) => {{
        ProjectsPhase::new()
            .workspace(WorkspaceMetadata {
                $($field: $val.into(),)*
                ..WorkspaceMetadata::default()
            })
            $(.project($proj))*
            .next()
            $(.feature($feat))*
            .next()
            $(.package($pkg))*
            .finish()
            .build()
    }};

    // Arm without workspace metadata (backwards compatible)
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
