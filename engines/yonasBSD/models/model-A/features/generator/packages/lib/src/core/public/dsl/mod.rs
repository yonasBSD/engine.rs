mod custom_modules;
pub mod features_phase;
pub mod final_phase;
pub mod macros;
pub mod node;
pub mod packages_phase;
pub mod projects_phase;

// Internal helper re-export so phases can stay clean
pub(crate) use custom_modules::insert_custom_module;

pub mod prelude {
    pub use crate::core::public::dsl::features_phase::FeaturesPhase;
    pub use crate::core::public::dsl::final_phase::FinalPhase;
    pub use crate::core::public::dsl::packages_phase::PackagesPhase;
    pub use crate::core::public::dsl::projects_phase::ProjectsPhase;
}

pub use node::*;
