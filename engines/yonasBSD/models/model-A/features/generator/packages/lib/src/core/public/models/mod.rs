// ==========================================================
// CONFIGURATION MODELS
// ==========================================================

mod tests;

use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    core::ConfigError,
    enums::DirSpec,
    prelude::*,
    utils::{default_features, default_packages, default_projects},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReadmeConfig {
    pub path: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_projects")]
    pub projects: Vec<String>,
    #[serde(default = "default_features")]
    pub features: Vec<String>,
    #[serde(default = "default_packages")]
    pub packages: Vec<String>,
    #[serde(default)]
    pub readmes: Vec<ReadmeConfig>,
    #[serde(default)]
    pub custom_modules: HashMap<String, DirSpec>,
    #[serde(default)]
    pub extra_folders: Vec<String>,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyList {
                field,
            } => {
                write!(f, "The list '{field}' cannot be empty.")
            },
            Self::DuplicateValues {
                field,
                duplicates,
            } => {
                write!(
                    f,
                    "The list '{field}' contains duplicate values: {duplicates:?}"
                )
            },
            Self::InvalidName {
                field,
                value,
            } => {
                write!(
                    f,
                    "Invalid name '{value}' in field '{field}'. Names must match ^[a-zA-Z0-9_-]+$"
                )
            },
        }
    }
}

fn check_reserved(
    field: &'static str,
    list: &[String],
    errors: &mut Vec<ConfigError>,
    reserved: &[&str],
) {
    for item in list {
        if reserved.contains(&item.as_str()) {
            errors.push(ConfigError::InvalidName {
                field,
                value: item.clone(),
            });
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    #[must_use]
    pub fn new(
        projects: Vec<String>,
        features: Vec<String>,
        packages: Vec<String>,
        readmes: Vec<ReadmeConfig>,
    ) -> Self {
        let custom_modules = dirs!({
            "api" => {
                "core": {
                    "backends": ["graphql", "grpc", "rest"],
                }
            },
        });

        Self {
            projects,
            features,
            packages,
            readmes,
            custom_modules,
            extra_folders: vec![],
        }
    }

    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();

        // 1. Empty lists
        if self.projects.is_empty() {
            errors.push(ConfigError::EmptyList {
                field: "projects",
            });
        }
        if self.features.is_empty() {
            errors.push(ConfigError::EmptyList {
                field: "features",
            });
        }
        if self.packages.is_empty() {
            errors.push(ConfigError::EmptyList {
                field: "packages",
            });
        }

        // 2. Duplicate detection helper
        fn find_duplicates(list: &[String]) -> Vec<String> {
            use std::collections::HashSet;
            let mut seen = HashSet::new();
            let mut dupes = HashSet::new();
            for item in list {
                if !seen.insert(item.clone()) {
                    dupes.insert(item.clone());
                }
            }
            dupes.into_iter().collect()
        }

        // 2a. Duplicates
        let d = find_duplicates(&self.projects);
        if !d.is_empty() {
            errors.push(ConfigError::DuplicateValues {
                field: "projects",
                duplicates: d,
            });
        }

        let d = find_duplicates(&self.features);
        if !d.is_empty() {
            errors.push(ConfigError::DuplicateValues {
                field: "features",
                duplicates: d,
            });
        }

        let d = find_duplicates(&self.packages);
        if !d.is_empty() {
            errors.push(ConfigError::DuplicateValues {
                field: "packages",
                duplicates: d,
            });
        }

        // 3. Invalid names
        let valid = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

        for p in &self.projects {
            if !valid.is_match(p) {
                errors.push(ConfigError::InvalidName {
                    field: "projects",
                    value: p.clone(),
                });
            }
        }

        for f in &self.features {
            if !valid.is_match(f) {
                errors.push(ConfigError::InvalidName {
                    field: "features",
                    value: f.clone(),
                });
            }
        }

        for pkg in &self.packages {
            if !valid.is_match(pkg) {
                errors.push(ConfigError::InvalidName {
                    field: "packages",
                    value: pkg.clone(),
                });
            }
        }

        // 4. Path collision detection
        let mut seen_paths = std::collections::HashSet::new();

        for project in &self.projects {
            for feature in &self.features {
                for package in &self.packages {
                    let path = format!(
                        "engines/{project}/models/model-A/features/{feature}/packages/{package}"
                    );

                    if !seen_paths.insert(path.clone()) {
                        errors.push(ConfigError::DuplicateValues {
                            field: "path",
                            duplicates: vec![path],
                        });
                    }
                }
            }
        }

        let reserved = [
            "core",
            "internal",
            "private",
            "public",
            "tests",
            "integration",
            "unit",
            "src",
            "packages",
            "models",
            "features",
            "engine",
            "engines",
        ];

        check_reserved("projects", &self.projects, &mut errors, &reserved);
        check_reserved("features", &self.features, &mut errors, &reserved);
        check_reserved("packages", &self.packages, &mut errors, &reserved);
        check_reserved("extra_folders", &self.extra_folders, &mut errors, &reserved);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
