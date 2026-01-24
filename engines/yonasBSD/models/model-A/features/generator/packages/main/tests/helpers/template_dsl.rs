use std::path::Path;

use crate::helpers::{ScaffolderTestHarness, read_file};

/// DSL for working with templates and generated files in tests.
///
/// This is intentionally minimal and focused on ergonomics.
pub trait TemplateDsl {
    /// Read a template file relative to the test root.
    fn read_template(&self, rel: impl AsRef<Path>) -> String;

    /// Read a generated file and assert it contains a given substring.
    fn assert_file_contains(&self, rel: &impl AsRef<Path>, needle: &str);
}

impl TemplateDsl for ScaffolderTestHarness {
    fn read_template(&self, rel: impl AsRef<Path>) -> String {
        read_file(self.path().join(rel))
    }

    fn assert_file_contains(&self, rel: &impl AsRef<Path>, needle: &str) {
        let content = read_file(self.path().join(rel));
        assert!(
            content.contains(needle),
            "Expected file {:?} to contain {:?}, but it did not.\n\nContent:\n{}",
            rel.as_ref(),
            needle,
            content
        );
    }
}
