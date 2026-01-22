use miette::SourceSpan;
use std::collections::HashMap;

use super::features_phase::FeaturesPhase;
use crate::ReadmeConfig;
use crate::core::public::dsl::{DslNode, insert_custom_module, default_span};
use crate::enums::DirSpec;

#[derive(Debug)]
pub struct ProjectsPhase {
    pub projects: Vec<DslNode<String>>,
    pub features: Vec<DslNode<String>>,
    pub packages: Vec<DslNode<String>>,
    pub readmes: Vec<DslNode<ReadmeConfig>>,
    pub custom_modules: HashMap<String, DirSpec>,
    pub extra_folders: Vec<DslNode<String>>,
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

    pub fn add_project(mut self, name: impl Into<String>, span: SourceSpan) -> Self {
        self.projects.push(DslNode::new(name.into(), span));
        self
    }

    // Convenience method for tests
    pub fn project(self, name: impl Into<String>) -> Self {
        self.add_project(name, default_span())
    }

    pub fn add_feature(mut self, name: impl Into<String>, span: SourceSpan) -> Self {
        self.features.push(DslNode::new(name.into(), span));
        self
    }

    pub fn feature(self, name: impl Into<String>) -> Self {
        self.add_feature(name, default_span())
    }

    pub fn add_package(mut self, name: impl Into<String>, span: SourceSpan) -> Self {
        self.packages.push(DslNode::new(name.into(), span));
        self
    }

    pub fn package(self, name: impl Into<String>) -> Self {
        self.add_package(name, default_span())
    }

    pub fn add_readme(mut self, file: impl Into<String>, path: impl Into<String>, span: SourceSpan) -> Self {
        self.readmes.push(DslNode::new(ReadmeConfig {
            file: file.into(),
            path: path.into(),
        }, span));
        self
    }

    pub fn readme(self, file: impl Into<String>, path: impl Into<String>) -> Self {
        self.add_readme(file, path, default_span())
    }

    /// Add a custom module using a dotted path and a list of backends.
    ///
    /// Example: `"api.core"`, &["graphql", "grpc", "rest"]`
    pub fn add_custom_module(mut self, path: impl AsRef<str>, backends: &[&str]) -> Self {
        insert_custom_module(&mut self.custom_modules, path.as_ref(), backends);
        self
    }

    pub fn add_extra_folder(mut self, folder: impl Into<String>, span: SourceSpan) -> Self {
        self.extra_folders.push(DslNode::new(folder.into(), span));
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
