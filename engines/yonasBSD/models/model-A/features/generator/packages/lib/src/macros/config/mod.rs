/// Declarative macro entry point for the config DSL.
///
/// This macro expands into the typed, staged DSL:
/// `ProjectsPhase` → `FeaturesPhase` → `PackagesPhase` → `FinalPhase` → Config
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
