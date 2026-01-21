use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::Path;

use engine_rs_lib::*;
use crate::helpers::*;

/// Trait providing all harness functionality.
/// Keeps the core struct clean and focused.
pub trait HarnessExtensions {
    fn write_config(&self, contents: &str);
    fn write_config_builder(&self, builder: ConfigBuilder);
    fn run(&self);
    fn run_init(&self);
    fn run_validate(&self);
    fn read(&self, rel: impl AsRef<Path>) -> String;
    fn tree(&self) -> String;
    fn assert_exists(&self, rel: impl AsRef<Path>);
}

impl HarnessExtensions for ScaffolderTestHarness {
    fn write_config(&self, contents: &str) {
        fs::write(self.path().join("config.toml"), contents).unwrap();
    }

    fn write_config_builder(&self, builder: ConfigBuilder) {
        self.write_config(&builder.build());
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
        read_file(self.path().join(rel))
    }

    fn tree(&self) -> String {
        capture_tree(self.path())
    }

    fn assert_exists(&self, rel: impl AsRef<Path>) {
        assert!(self.path().join(rel).exists());
    }
}
