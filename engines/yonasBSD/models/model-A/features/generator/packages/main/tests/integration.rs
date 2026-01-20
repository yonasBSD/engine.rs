use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use engine_rs::{core::Config, core::ReadmeConfig, traits::FileSystem, traits::Scaffolder};

// ==========================================================
// THE NON-BLOCKING MOCK FILESYSTEM
// ==========================================================
#[derive(Clone, Default)]
struct MockFS {
    files: Rc<RefCell<HashMap<PathBuf, String>>>,
    dirs: Rc<RefCell<Vec<PathBuf>>>,
}

impl FileSystem for MockFS {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        self.dirs.borrow_mut().push(path.to_path_buf());
        Ok(())
    }

    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        self.files
            .borrow_mut()
            .insert(path.to_path_buf(), content.to_string());
        Ok(())
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found in MockFS"))
    }
}

// ==========================================================
// INTEGRATION TESTS
// ==========================================================
#[test]
fn test_scaffolder_full_integration() {
    let mock = MockFS::default();
    let base_path = PathBuf::from("/mock_home");
    let scaffolder = Scaffolder::new(mock.clone(), base_path.clone());

    let config = Config {
        project_name: "apollo".to_string(),
        feature_name: "auth".to_string(),
        package_name: "jwt".to_string(),
        readme: vec![ReadmeConfig {
            path: "engines/apollo".to_string(),
            file: "readme/engines.md.tpl".to_string(),
        }],
    };

    // Run Scaffolder
    let manifest = scaffolder.run(config).expect("Scaffolding failed");

    // Verification
    let files = mock.files.borrow();

    // Check Mirrored Hierarchy
    let backend_unit = base_path.join(
        "engines/apollo/models/model-A/features/auth/packages/jwt/core/backends/tests/unit/mod.rs",
    );
    let frontend_unit = base_path.join(
        "engines/apollo/models/model-A/features/auth/packages/jwt/core/frontends/tests/unit/mod.rs",
    );

    assert!(
        files.contains_key(&backend_unit),
        "Backend unit test file missing"
    );
    assert!(
        files.contains_key(&frontend_unit),
        "Frontend unit test file missing"
    );

    // Verify template rendering via BLAKE3
    let integrity = scaffolder.verify_integrity(manifest);
    assert!(
        integrity.is_ok(),
        "Integrity check failed on valid mock files"
    );
}
