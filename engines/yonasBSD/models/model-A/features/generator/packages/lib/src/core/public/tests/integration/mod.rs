#[cfg(test)]
mod loom_tests {
    //! Loom concurrency tests wired against the real engine types:
    //! - Config / ReadmeConfig
    //! - FileSystem trait
    //! - Scaffolder engine

    use crate::{core::Config, core::ReadmeConfig, traits::FileSystem, traits::Scaffolder};

    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::sync::{Arc, Mutex};
    use loom::thread;

    use std::collections::HashMap;
    use std::io;
    use std::path::{Path, PathBuf};

    //
    // 0. LOOM-AWARE FILESYSTEM IMPLEMENTATION
    //

    #[derive(Clone)]
    struct LoomFS {
        files: Arc<Mutex<HashMap<PathBuf, String>>>,
        dirs: Arc<Mutex<Vec<PathBuf>>>,
        create_dir_calls: Arc<AtomicUsize>,
        write_file_calls: Arc<AtomicUsize>,
    }

    impl LoomFS {
        fn new() -> Self {
            Self {
                files: Arc::new(Mutex::new(HashMap::new())),
                dirs: Arc::new(Mutex::new(Vec::new())),
                create_dir_calls: Arc::new(AtomicUsize::new(0)),
                write_file_calls: Arc::new(AtomicUsize::new(0)),
            }
        }
    }

    impl FileSystem for LoomFS {
        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            self.create_dir_calls.fetch_add(1, Ordering::SeqCst);
            let mut dirs = self.dirs.lock().unwrap();
            dirs.push(path.to_path_buf());
            Ok(())
        }

        fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
            self.write_file_calls.fetch_add(1, Ordering::SeqCst);
            let mut files = self.files.lock().unwrap();
            files.insert(path.to_path_buf(), content.to_string());
            Ok(())
        }

        fn read_to_string(&self, path: &Path) -> io::Result<String> {
            let files = self.files.lock().unwrap();
            files
                .get(path)
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
        }
    }

    //
    // 1. GLOBAL STATE INITIALIZATION (PURE LOOM)
    //

    #[test]
    fn loom_global_init() {
        loom::model(|| {
            let init_count = Arc::new(AtomicUsize::new(0));

            let spawn_init = || {
                let counter = Arc::clone(&init_count);
                thread::spawn(move || {
                    let prev = counter.fetch_add(1, Ordering::SeqCst);
                    if prev == 0 {
                        // first initializer wins
                    }
                })
            };

            let t1 = spawn_init();
            let t2 = spawn_init();
            let t3 = spawn_init();

            t1.join().unwrap();
            t2.join().unwrap();
            t3.join().unwrap();

            let count = init_count.load(Ordering::SeqCst);
            assert!((1..=3).contains(&count));
        });
    }

    //
    // 2. SCAFFOLDER + FILESYSTEM UNDER LOOM
    //

    #[test]
    fn loom_scaffolder_single_run_is_consistent() {
        loom::model(|| {
            let fs = LoomFS::new();
            let base = PathBuf::from("/virtual");

            let scaffolder = Scaffolder::new(fs.clone(), base.clone());

            let config = Config {
                projects: vec!["loom-project".to_string()],
                features: vec!["loom-feature".to_string()],
                packages: vec!["loom-package".to_string()],
                readme: vec![ReadmeConfig {
                    path: "engines/loom-project".to_string(),
                    file: "internal/readme.tpl".to_string(),
                }],
            };

            let manifest = scaffolder
                .run(config.clone())
                .expect("scaffolder run should succeed under loom");

            // Verify integrity using the same LoomFS
            let result = scaffolder.verify_integrity(manifest);

            assert!(
                result.is_ok(),
                "verify_integrity should succeed under all explored schedules"
            );

            // Sanity: we actually created some dirs/files
            assert!(fs.create_dir_calls.load(Ordering::SeqCst) > 0);
            assert!(fs.write_file_calls.load(Ordering::SeqCst) > 0);
        });
    }

    //
    // 3. MULTI-THREADED LOG EMISSION-LIKE PATTERN USING FILESYSTEM
    //

    #[test]
    fn loom_multi_threaded_writes_do_not_corrupt_state() {
        loom::model(|| {
            let fs = LoomFS::new();
            let base = PathBuf::from("/virtual");

            // Create two independent scaffolders using the same LoomFS
            let s1 = Scaffolder::new(fs.clone(), base.clone());
            let s2 = Scaffolder::new(fs.clone(), base.clone());

            let config = Config {
                projects: vec!["loom-project".to_string()],
                features: vec!["loom-feature".to_string()],
                packages: vec!["pkg-a".to_string(), "pkg-b".to_string()],
                readme: vec![ReadmeConfig {
                    path: "engines/loom-project".to_string(),
                    file: "internal/readme.tpl".to_string(),
                }],
            };

            let cfg1 = config.clone();
            let cfg2 = config.clone();

            let t1 = thread::spawn(move || {
                let _ = s1.run(cfg1);
            });

            let t2 = thread::spawn(move || {
                let _ = s2.run(cfg2);
            });

            t1.join().unwrap();
            t2.join().unwrap();

            // Validate filesystem state
            let files = fs.files.lock().unwrap();
            for (path, content) in files.iter() {
                assert!(
                    !content.is_empty(),
                    "file {:?} should not be empty under any schedule",
                    path
                );
            }
        });
    }
}
