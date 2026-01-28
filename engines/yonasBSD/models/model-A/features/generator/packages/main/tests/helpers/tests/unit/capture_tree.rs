use std::fs;

use tempfile::tempdir;

use crate::helpers::capture_tree;

#[test]
fn capture_tree_is_deterministic() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create files and dirs in intentionally scrambled order
    let paths = [
        "zeta.txt",
        "alpha/",
        "alpha/beta.txt",
        "alpha/zzz/",
        "alpha/zzz/inner.txt",
        "gamma/",
        "gamma/a.txt",
    ];

    // Create them in reverse order to maximize nondeterminism
    for p in paths.iter().rev() {
        let full = root.join(p);
        if p.ends_with('/') {
            fs::create_dir_all(&full).unwrap();
        } else {
            if let Some(parent) = full.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&full, "x").unwrap();
        }
    }

    let output = capture_tree(root);

    // Expected sorted output:
    // - directories first
    // - alphabetical within each group
    // - no root entry
    let expected = [
        "alpha",
        "alpha/zzz",
        "gamma",
        "alpha/beta.txt",
        "alpha/zzz/inner.txt",
        "gamma/a.txt",
        "zeta.txt",
    ]
    .join("\n")
        + "\n";

    assert_eq!(output, expected);
}
