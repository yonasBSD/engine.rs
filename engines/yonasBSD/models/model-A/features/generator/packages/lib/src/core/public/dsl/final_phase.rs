use std::collections::HashMap;

use crate::{
    ReadmeConfig, WorkspaceMetadata,
    core::public::{Config, dsl::DslNode},
    enums::DirSpec,
};

#[derive(Debug)]
pub struct FinalPhase {
    pub workspace: WorkspaceMetadata,
    pub projects: Vec<DslNode<String>>,
    pub features: Vec<DslNode<String>>,
    pub packages: Vec<DslNode<String>>,
    pub readmes: Vec<DslNode<ReadmeConfig>>,
    pub custom_modules: HashMap<String, DirSpec>,
    pub extra_folders: Vec<DslNode<String>>,
}

impl FinalPhase {
    #[must_use]
    pub fn build(self) -> Config {
        Config {
            workspace: self.workspace,
            projects: self.projects.into_iter().map(|n| n.value).collect(),
            features: self.features.into_iter().map(|n| n.value).collect(),
            packages: self.packages.into_iter().map(|n| n.value).collect(),
            readmes: self.readmes.into_iter().map(|n| n.value).collect(),
            custom_modules: self.custom_modules,
            extra_folders: self.extra_folders.into_iter().map(|n| n.value).collect(),
        }
    }
}
