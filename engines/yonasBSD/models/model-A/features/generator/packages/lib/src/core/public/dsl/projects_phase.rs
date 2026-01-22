use crate::ReadmeConfig;
use crate::core::public::dsl::insert_custom_module;
use crate::enums::DirSpec;
use std::collections::HashMap;

use super::features_phase::FeaturesPhase;

#[derive(Debug)]
pub struct ProjectsPhase {
    pub projects: Vec<String>,
    pub features: Vec<String>,
    pub packages: Vec<String>,
    pub readmes: Vec<ReadmeConfig>,
    pub custom_modules: HashMap<String, DirSpec>,
    pub extra_folders: Vec<String>,
}

impl Default for ProjectsPhase {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectsPhase {
    #[must_use]
    pub fn new() -> Self {
        Self {
            projects: vec![],
            features: vec![],
            packages: vec![],
            readmes: vec![],
            custom_modules: HashMap::new(),
            extra_folders: vec![],
        }
    }

    pub fn add_project(mut self, name: impl Into<String>) -> Self {
        self.projects.push(name.into());
        self
    }

    pub fn add_readme(mut self, file: impl Into<String>, path: impl Into<String>) -> Self {
        self.readmes.push(ReadmeConfig {
            file: file.into(),
            path: path.into(),
        });
        self
    }

    /// Add a custom module using a dotted path and a list of backends.
    ///
    /// Example: `"api.core"`, &["graphql", "grpc", "rest"]`
    pub fn add_custom_module(mut self, path: impl AsRef<str>, backends: &[&str]) -> Self {
        insert_custom_module(&mut self.custom_modules, path.as_ref(), backends);
        self
    }

    pub fn add_extra_folder(mut self, folder: impl Into<String>) -> Self {
        self.extra_folders.push(folder.into());
        self
    }

    #[must_use]
    pub fn next(self) -> FeaturesPhase {
        FeaturesPhase {
            projects: self.projects,
            features: self.features,
            packages: self.packages,
            readmes: self.readmes,
            custom_modules: self.custom_modules,
            extra_folders: self.extra_folders,
        }
    }
}
