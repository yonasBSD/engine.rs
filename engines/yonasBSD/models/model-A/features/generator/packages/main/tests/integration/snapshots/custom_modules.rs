use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;
use std::fs;

use crate::helpers::{capture_tree, read_file};

#[test]
fn snapshot_custom_module_tree() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    // Write config.toml
    fs::write(
        root.join("config.toml"),
        r#"
            projects = ["demo"]
            features = ["alpha"]
            packages = ["api"]

            [custom_modules.api.core]
            backends = ["graphql", "grpc", "rest"]
        "#,
    )
    .unwrap();

    // Run scaffolder
    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("run")
        .assert()
        .success();

    // Capture directory tree
    let tree = capture_tree(root);

    insta::assert_snapshot!("custom_module_tree", tree);

    // Snapshot a specific file (example)
    let mod_rs = read_file(
        &root.join("engines/demo/models/model-A/features/alpha/packages/api/core/backends/mod.rs")
    );

    insta::assert_snapshot!("backends_mod_rs", mod_rs);
}
