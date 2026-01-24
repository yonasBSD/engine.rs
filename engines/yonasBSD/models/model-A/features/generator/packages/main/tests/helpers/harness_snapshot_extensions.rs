use crate::helpers::{ScaffolderTestHarness, capture_tree, read_file};

/// Extensions focused on snapshot ergonomics.
///
/// These are intentionally thin wrappers around insta::assert_snapshot!
pub trait HarnessSnapshotExtensions {
    /// Snapshot the entire directory tree with a given snapshot name.
    fn snapshot_tree(&self, name: &str);

    /// Snapshot a single file with a given snapshot name.
    fn snapshot_file(&self, name: &str, rel: &str);
}

impl HarnessSnapshotExtensions for ScaffolderTestHarness {
    fn snapshot_tree(&self, name: &str) {
        let tree = capture_tree(self.path());
        insta::assert_snapshot!(name, tree);
    }

    fn snapshot_file(&self, name: &str, rel: &str) {
        let content = read_file(self.path().join(rel));
        insta::assert_snapshot!(name, content);
    }
}
