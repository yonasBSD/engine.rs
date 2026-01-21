use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;
use std::fs;

use crate::helpers::{read_file, capture_tree};

#[test]
fn snapshot_readme_templated_paths() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    fs::write(
        root.join("config.toml"),
        r#"
            projects = ["demo"]
            features = ["alpha"]
            packages = ["api", "lib"]

            # Global README
            [[readme]]
            file = "readme/example.md.tpl"
            path = "engines/{{ project }}"

            # README for api package (path determines target)
            [[readme]]
            file = "readme/api.md.tpl"
            path = "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/api"

            # README for lib package (path determines target)
            [[readme]]
            file = "readme/lib.md.tpl"
            path = "engines/{{ project }}/models/{{ model }}/features/{{ feature }}/packages/lib"
        "#,
    ).unwrap();

    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("run")
        .assert()
        .success();

    // Snapshot the tree
    let tree = capture_tree(root);
    insta::assert_snapshot!("templated_path_tree", tree);

    // Snapshot README contents
    let global = read_file(
        &root.join("engines/demo/README.md")
    );

    let api = read_file(
        &root.join("engines/demo/models/model-A/features/alpha/packages/api/README.md")
    );

    let lib = read_file(
        &root.join("engines/demo/models/model-A/features/alpha/packages/lib/README.md")
    );

    insta::assert_snapshot!("readme_global", global);
    insta::assert_snapshot!("readme_api", api);
    insta::assert_snapshot!("readme_lib", lib);
}
