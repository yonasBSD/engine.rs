use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::{tempdir, TempDir};
use std::fs;
use std::path::Path;

use crate::helpers::{capture_tree, read_file};

pub struct ScaffolderTestHarness {
    pub root: TempDir,
}

impl ScaffolderTestHarness {
    /// Create a new isolated test environment
    pub fn new() -> Self {
        Self {
            root: tempdir().expect("failed to create tempdir"),
        }
    }

    /// Path helper
    pub fn path(&self) -> &Path {
        self.root.path()
    }

    /// Write a config.toml into the test root
    pub fn write_config(&self, contents: &str) {
        fs::write(self.path().join("config.toml"), contents)
            .expect("failed to write config.toml");
    }

    /// Run the scaffolder CLI (`engine-rs run`)
    pub fn run(&self) {
        cargo_bin_cmd!("engine-rs")
            .current_dir(self.path())
            .arg("run")
            .assert()
            .success();
    }

    /// Read a generated file relative to the test root
    pub fn read(&self, rel: impl AsRef<Path>) -> String {
        read_file(&self.path().join(rel))
    }

    /// Capture the entire directory tree for snapshot testing
    pub fn tree(&self) -> String {
        capture_tree(self.path())
    }

    /// Assert that a file exists
    pub fn assert_exists(&self, rel: impl AsRef<Path>) {
        let full = self.path().join(rel);
        assert!(
            full.exists(),
            "Expected file to exist: {}",
            full.display()
        );
    }

    /// Assert that a file does NOT exist
    pub fn assert_not_exists(&self, rel: impl AsRef<Path>) {
        let full = self.path().join(rel);
        assert!(
            !full.exists(),
            "Expected file to NOT exist: {}",
            full.display()
        );
    }
}
