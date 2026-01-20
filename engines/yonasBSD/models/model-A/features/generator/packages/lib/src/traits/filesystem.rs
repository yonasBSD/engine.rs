// ==========================================================
// FILESYSTEM ABSTRACTION
// ==========================================================
use crate::core::*;

use blake3;
use minijinja::{Environment, context};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()>;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
}

pub struct RealFS;
impl FileSystem for RealFS {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }
}

// ==========================================================
// 4. THE SCAFFOLDER ENGINE
// ==========================================================
pub struct Scaffolder<F: FileSystem> {
    pub fs: F,
    pub base_path: PathBuf,
    pub env: Environment<'static>,
}

impl<F: FileSystem> Scaffolder<F> {
    pub fn new(fs: F, base_path: PathBuf) -> Self {
        let mut env = Environment::new();

        // Load all embedded templates and shared vars
        for file in Asset::iter() {
            if let Some(content) = Asset::get(&file) {
                let body = std::str::from_utf8(content.data.as_ref())
                    .unwrap()
                    .to_owned();
                let name: &'static str = Box::leak(file.to_string().into_boxed_str());
                let body_str: &'static str = Box::leak(body.into_boxed_str());
                env.add_template(name, body_str).unwrap();
            }
        }

        // Register Internal Fallbacks
        let _ = env.add_template("internal/readme.tpl", DEFAULT_README_TPL);
        let _ = env.add_template("internal/cargo.tpl", TPL_CARGO);
        let _ = env.add_template("internal/mod_export.tpl", TPL_MOD_EXPORT);
        let _ = env.add_template("internal/mod_tests.tpl", TPL_MOD_TESTS);

        Self { fs, base_path, env }
    }

    pub fn calculate_hash(&self, content: &str) -> String {
        blake3::hash(content.as_bytes()).to_hex().to_string()
    }

    pub fn run(&self, config: Config) -> io::Result<HashMap<PathBuf, String>> {
        let mut manifest = HashMap::new();
        let ctx = context!(
            project_name => config.project_name,
            feature_name => config.feature_name,
            package_name => config.package_name
        );

        let root = self
            .base_path
            .join(format!("engines/{}", config.project_name));
        let pkg_path = root.join(format!(
            "models/model-A/features/{}/packages/{}",
            config.feature_name, config.package_name
        ));

        // 1. Generate READMEs
        for entry in config.readme {
            let tpl_name = if self.env.get_template(&entry.file).is_ok() {
                &entry.file
            } else {
                "internal/readme.tpl"
            };
            let rendered = self
                .env
                .get_template(tpl_name)
                .unwrap()
                .render(ctx.clone())
                .unwrap();
            let dest = self.base_path.join(&entry.path).join("README.md");
            self.fs.create_dir_all(dest.parent().unwrap())?;
            self.fs.write_file(&dest, &rendered)?;
            manifest.insert(dest, self.calculate_hash(&rendered));
        }

        // 2. Base Project Structure
        self.fs.create_dir_all(&pkg_path.join("src"))?;

        let cargo_path = root.join("models/model-A/features/Cargo.toml");
        let cargo_content = self
            .env
            .get_template("internal/cargo.tpl")
            .unwrap()
            .render(ctx.clone())
            .unwrap();
        self.fs.create_dir_all(cargo_path.parent().unwrap())?;
        self.fs.write_file(&cargo_path, &cargo_content)?;
        manifest.insert(cargo_path, self.calculate_hash(&cargo_content));

        let mod_path = pkg_path.join("mod.rs");
        let mod_content = self
            .env
            .get_template("internal/mod_export.tpl")
            .unwrap()
            .render(ctx.clone())
            .unwrap();
        self.fs.write_file(&mod_path, &mod_content)?;
        manifest.insert(mod_path, self.calculate_hash(&mod_content));

        // 3. Mirrored Module Generation
        let modules = [
            "enums",
            "macros",
            "utils",
            "traits",
            "core/backends",
            "core/frontends",
            "core/public",
            "core/internal",
            "core/private",
        ];

        for mod_dir in modules {
            let base = pkg_path.join(mod_dir);
            let src_dir = base.join("src");
            let tests_mod = base.join("tests/mod.rs");
            let unit_mod = base.join("tests/unit/mod.rs");

            self.fs.create_dir_all(&src_dir)?;
            self.fs.create_dir_all(unit_mod.parent().unwrap())?;

            self.fs
                .write_file(&base.join("mod.rs"), "pub mod tests;\n")?;

            let t_content = self
                .env
                .get_template("internal/mod_tests.tpl")
                .unwrap()
                .render(ctx.clone())
                .unwrap();
            self.fs.write_file(&tests_mod, &t_content)?;
            self.fs.write_file(&unit_mod, "// Automated Unit Tests\n")?;

            manifest.insert(tests_mod, self.calculate_hash(&t_content));
            manifest.insert(unit_mod, self.calculate_hash("// Automated Unit Tests\n"));
        }

        Ok(manifest)
    }

    pub fn verify_integrity(
        &self,
        manifest: HashMap<PathBuf, String>,
    ) -> Result<Duration, Vec<String>> {
        let start = Instant::now();
        let mut errors = Vec::new();
        for (path, expected_hash) in manifest {
            match self.fs.read_to_string(&path) {
                Ok(content) => {
                    if self.calculate_hash(&content) != expected_hash {
                        errors.push(format!("Hash Mismatch: {:?}", path));
                    }
                }
                Err(_) => errors.push(format!("File Missing: {:?}", path)),
            }
        }
        if errors.is_empty() {
            Ok(start.elapsed())
        } else {
            Err(errors)
        }
    }
}
