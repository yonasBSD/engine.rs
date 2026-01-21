use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::PathBuf;

use engine_rs_lib::{core::*, traits::*};

#[test]
fn custom_modules_are_generated() {
    // 1. Create a temp directory
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    // 2. Write a config.toml with custom modules
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

    // 3. Run the scaffolder
    cargo_bin_cmd!("engine-rs")
        .current_dir(root)
        .arg("run")
        .assert()
        .success();

    // 4. Assert the custom module tree exists
    let base: PathBuf =
        root.join("engines/demo/models/model-A/features/alpha/packages/api/core/backends");

    for backend in ["graphql", "grpc", "rest"] {
        let dir = base.join(backend);

        assert!(
            dir.exists(),
            "backend directory missing: {:?}",
            dir.display()
        );

        assert!(
            dir.join("tests/mod.rs").exists(),
            "tests/mod.rs missing for {:?}",
            backend
        );
        assert!(
            dir.join("tests/unit/mod.rs").exists(),
            "unit tests missing for {:?}",
            backend
        );
        assert!(
            dir.join("tests/integration/mod.rs").exists(),
            "integration tests missing for {:?}",
            backend
        );
        assert!(
            dir.join("mod.rs").exists(),
            "mod.rs missing for {:?}",
            backend
        );
    }

    // 5. Verify manifest integrity
    let fs = RealFS;
    let scaffolder = Scaffolder::new(fs, root.to_path_buf());

    let config: Config =
        toml::from_str(&fs::read_to_string(root.join("config.toml")).unwrap()).unwrap();

    let manifest = scaffolder.run(config).unwrap();

    let result = scaffolder.verify_integrity(manifest);
    assert!(result.is_ok(), "manifest integrity failed");
}
