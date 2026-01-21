#[test]
fn custom_modules_are_generated() {
    use engine_rs_lib::{DirSpec, core::*, dirs, traits::*};

    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();

    // 1. Build a config with custom modules
    let config = Config {
        projects: vec!["demo".into()],
        features: vec!["alpha".into()],
        packages: vec!["api".into()],
        readme: vec![],
        custom_modules: dirs!({
            "api" => {
                "core": {
                    "backends": ["graphql", "grpc", "rest"]
                }
            }
        }),
        extra_folders: vec![],
    };

    // 2. Run the scaffolder
    let fs = RealFS;
    let scaffolder = Scaffolder::new(fs, root.clone());
    let manifest = scaffolder.run(config).expect("scaffolder failed");

    // 3. Assert directories exist
    let base = root.join("engines/demo/models/model-A/features/alpha/packages/api/core/backends");

    for backend in ["graphql", "grpc", "rest"] {
        let dir = base.join(backend);
        assert!(dir.exists(), "backend directory missing: {:?}", dir);

        assert!(
            dir.join("tests/mod.rs").exists(),
            "tests missing for {:?}",
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

    // 4. Verify manifest integrity
    let duration = scaffolder
        .verify_integrity(manifest)
        .expect("manifest integrity failed");

    assert!(duration.as_millis() > 0);
}
