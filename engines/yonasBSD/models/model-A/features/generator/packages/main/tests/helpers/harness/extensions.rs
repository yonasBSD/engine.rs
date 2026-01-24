use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
use assert_fs::{fixture::ChildPath, prelude::*};
use engine_rs_lib::*;
use predicates::prelude::*;

use crate::helpers::*;

/// Trait providing all harness functionality.
/// Keeps the core struct clean and focused.
pub trait HarnessExtensions {
    fn write_config(&self, cfg: &Config);
    fn run(&self);
    fn run_init(&self);
    fn run_validate(&self);
    fn read(&self, rel: impl AsRef<Path>) -> String;
    fn tree(&self) -> String;
    fn assert_exists(&self, rel: impl AsRef<Path>);
    fn assert_all_readmes_exist(&self, cfg: &Config) -> Result<(), EngineError>;
}

impl HarnessExtensions for ScaffolderTestHarness {
    fn write_config(&self, cfg: &Config) {
        let contents = toml::to_string_pretty(cfg).expect("Config must be serializable to TOML");

        ChildPath::new(self.path().join("config.toml"))
            .write_str(&contents)
            .expect("failed to write config.toml");
    }

    fn run(&self) {
        cargo_bin_cmd!("engine-rs")
            .current_dir(self.path())
            .arg("run")
            .assert()
            .success();
    }

    fn run_init(&self) {
        cargo_bin_cmd!("engine-rs")
            .current_dir(self.path())
            .arg("init")
            .assert()
            .success();
    }

    fn run_validate(&self) {
        cargo_bin_cmd!("engine-rs")
            .current_dir(self.path())
            .arg("validate")
            .assert()
            .success();
    }

    fn read(&self, rel: impl AsRef<Path>) -> String {
        let file = ChildPath::new(self.path().join(rel.as_ref()));

        file.assert(predicate::path::is_file());
        std::fs::read_to_string(&file)
            .expect("failed to read file content after successful assertion")
    }

    fn tree(&self) -> String {
        capture_tree(self.path())
    }

    fn assert_exists(&self, rel: impl AsRef<Path>) {
        let child = ChildPath::new(self.path().join(rel));
        child.assert(predicate::path::exists());
    }

    fn assert_all_readmes_exist(&self, cfg: &Config) -> Result<(), EngineError> {
        use engine_rs_lib::utils::path_normalization::normalize_path_str;
        use minijinja::{Environment, context};

        let env = Environment::new();

        for project in &cfg.projects {
            for feature in &cfg.features {
                for package in &cfg.packages {
                    let ctx = context!(
                        project => project,
                        feature => feature,
                        package => package,
                        model => "model-A",
                    );

                    for readme in &cfg.readmes {
                        let rendered = env
                            .render_str(&readme.path, ctx.clone())
                            .expect("failed to render readme path");

                        let normalized = normalize_path_str(&rendered);

                        let target_dir = self.path().join(&normalized);
                        let readme_file = target_dir.join("README.md");

                        if !target_dir.exists() {
                            Err(ScaffolderError::DirectoryMissing {
                                path: target_dir.display().to_string(),
                            })?;
                        }

                        if !readme_file.is_file() {
                            Err(ScaffolderError::ReadmeMissing {
                                path: readme_file.display().to_string(),
                            })?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
