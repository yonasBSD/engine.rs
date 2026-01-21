use crate::helpers::ScaffolderTestHarness;
use crate::helpers::capture_tree;

/// DSL for asserting properties about the generated directory tree.
pub trait TreeAssertionDsl {
    /// Capture the tree and assert it contains a given substring.
    fn assert_tree_contains(&self, needle: &str);

    /// Capture the tree and return it (for chaining or custom assertions).
    fn tree_string(&self) -> String;
}

impl TreeAssertionDsl for ScaffolderTestHarness {
    fn assert_tree_contains(&self, needle: &str) {
        let tree = capture_tree(self.path());
        assert!(
            tree.contains(needle),
            "Expected tree to contain {:?}, but it did not.\n\nTree:\n{}",
            needle,
            tree
        );
    }

    fn tree_string(&self) -> String {
        capture_tree(self.path())
    }
}
