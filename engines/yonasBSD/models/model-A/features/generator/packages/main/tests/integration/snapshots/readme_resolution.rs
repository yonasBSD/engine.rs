use assert_cmd::cargo::cargo_bin_cmd;
use tempfile::tempdir;
use std::fs;

use crate::helpers::read_file;

#[test]
fn snapshot_readme_template_resolution() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    fs::write(
        root.join("config.toml"),
        r#"
            projects = ["demo"]
            features = ["alpha"]
            packages = ["api"]

            [[readme]]
            path = "engines/demo"
            file = "readme/example.md.tpl"

            [[readme]]
            path = "engines/demo/models"
            file = "readme/benches.md.tpl"
        "#,
    ).unwrap();

    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("run")
        .assert()
        .success();

    let readme1 = read_file(&root.join("engines/demo/README.md"));
    let readme2 = read_file(&root.join("engines/demo/models/README.md"));

    insta::assert_snapshot!("readme_demo", readme1);
    insta::assert_snapshot!("readme_models", readme2);
}
