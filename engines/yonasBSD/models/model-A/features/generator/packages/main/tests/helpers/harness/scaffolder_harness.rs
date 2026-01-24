use std::path::Path;

use tempfile::{TempDir, tempdir};

/// Core harness struct. Extensions live in `extensions.rs`.
pub struct ScaffolderTestHarness {
    pub root: TempDir,
}

impl ScaffolderTestHarness {
    pub fn new() -> Self {
        Self {
            root: tempdir().unwrap(),
        }
    }

    pub fn path(&self) -> &Path {
        self.root.path()
    }
}
