// ==========================================================
// FILESYSTEM ABSTRACTION
// ==========================================================
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use blake3;
use minijinja::{Environment, context};

use crate::{
    core::{
        Asset, Config, DEFAULT_README_TPL, EXTRA_TOP_LEVEL_DIRS, TPL_CARGO, TPL_MOD_EXPORT,
        TPL_MOD_TESTS,
    },
    enums::DirSpec,
};

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
// THE SCAFFOLDER ENGINE
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

        Self {
            fs,
            base_path,
            env,
        }
    }

    pub fn calculate_hash(&self, content: &str) -> String {
        blake3::hash(content.as_bytes()).to_hex().to_string()
    }

    pub fn run(&self, config: &Config) -> io::Result<HashMap<PathBuf, String>> {
        let mut manifest = HashMap::new();

        // Default sub-packages that always exist per feature
        let default_packages = ["cli", "api", "lib", "testing"];

        // Cartesian product: projects × features × packages
        for project in &config.projects {
            let project_root = self.base_path.join(format!("engines/{project}"));

            for feature in &config.features {
                let feature_root = project_root.join(format!("models/model-A/features/{feature}"));
                let packages_root = feature_root.join("packages");

                // 1. Generate Cargo.toml for this feature
                {
                    let ctx = context!(
                        project_name => project,
                        feature_name => feature,
                        // package_name is not needed for Cargo.toml, but we keep it
                        // available for templates that might reference it.
                        package_name => ""
                    );

                    let cargo_path = feature_root.join("Cargo.toml");
                    let cargo_content = self
                        .env
                        .get_template("internal/cargo.tpl")
                        .unwrap()
                        .render(ctx.clone())
                        .unwrap();

                    self.fs.create_dir_all(cargo_path.parent().unwrap())?;
                    self.fs.write_file(&cargo_path, &cargo_content)?;
                    manifest.insert(cargo_path, self.calculate_hash(&cargo_content));
                }

                // 2. Generate default sub-packages (cli, api, lib, testing)
                for default_pkg in default_packages {
                    let pkg_path = packages_root.join(default_pkg);
                    self.generate_package_structure(
                        &mut manifest,
                        config,
                        project,
                        feature,
                        default_pkg,
                        &pkg_path,
                    )?;
                }

                // 3. Generate user-defined packages
                for package in &config.packages {
                    let pkg_path = packages_root.join(package);
                    self.generate_package_structure(
                        &mut manifest,
                        config,
                        project,
                        feature,
                        package,
                        &pkg_path,
                    )?;
                }
            }
        }

        Ok(manifest)
    }

    fn generate_package_structure(
        &self,
        manifest: &mut HashMap<PathBuf, String>,
        config: &Config,
        project: &str,
        feature: &str,
        package: &str,
        pkg_path: &Path,
    ) -> io::Result<()> {
        // Context for templates
        let ctx = context!(
            project => project,
            feature => feature,
            package => package,
            model => "model-A",
        );

        // 1. Generate READMEs for this project/feature/package combo
        for entry in &config.readmes {
            // Resolve template name (your existing logic)
            let tpl_name = self
                .resolve_template_name(&entry.file)
                .unwrap_or_else(|| "internal/readme.tpl".to_string());

            // Render README content
            let rendered = self
                .env
                .get_template(&tpl_name)
                .unwrap()
                .render(ctx.clone())
                .unwrap();

            // Expand template variables
            let rendered_path = self.render_path(&entry.path, &ctx);

            // Normalize the path (remove duplicate slashes, resolve .., etc.)
            let normalized = PathBuf::from(rendered_path)
                .components()
                .collect::<PathBuf>();

            // Anchor to base_path
            let dest = self.base_path.join(normalized).join("README.md");

            self.fs.create_dir_all(dest.parent().unwrap())?;
            self.fs.write_file(&dest, &rendered)?;
            manifest.insert(dest, self.calculate_hash(&rendered));
        }

        // 2. Base package structure (src + tests)
        let src_dir = pkg_path.join("src");
        self.fs.create_dir_all(&src_dir)?;

        let mod_path = pkg_path.join("mod.rs");
        let mod_content = self
            .env
            .get_template("internal/mod_export.tpl")
            .unwrap()
            .render(ctx.clone())
            .unwrap();
        self.fs.write_file(&mod_path, &mod_content)?;
        manifest.insert(mod_path, self.calculate_hash(&mod_content));

        // 3. Mirrored module generation
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
            let base = pkg_path.join(format!("src/{mod_dir}"));
            let tests_mod = base.join("tests/mod.rs");
            let unit_mod = base.join("tests/unit/mod.rs");
            let integration_mod = base.join("tests/integration/mod.rs");

            self.fs.create_dir_all(unit_mod.parent().unwrap())?;
            self.fs.create_dir_all(integration_mod.parent().unwrap())?;

            self.fs.write_file(&base.join("mod.rs"), "mod tests;\n")?;

            let t_content = self
                .env
                .get_template("internal/mod_tests.tpl")
                .unwrap()
                .render(ctx.clone())
                .unwrap();

            self.fs.write_file(&tests_mod, &t_content)?;
            self.fs.write_file(&unit_mod, "// Automated Unit Tests\n")?;
            self.fs
                .write_file(&integration_mod, "// Automated Integration Tests\n")?;

            manifest.insert(tests_mod, self.calculate_hash(&t_content));
            manifest.insert(unit_mod, self.calculate_hash("// Automated Unit Tests\n"));
            manifest.insert(
                integration_mod,
                self.calculate_hash("// Automated Integration Tests\n"),
            );
        }

        // 4. Extra directories
        for extra in EXTRA_TOP_LEVEL_DIRS {
            self.fs.create_dir_all(&pkg_path.join(extra))?;
        }

        // 5. Custom module expansions (JSON-like DSL)
        if let Some(spec) = config.custom_modules.get(package) {
            self.generate_custom_tree(manifest, pkg_path, spec)?;
        }

        // 6. Custom folders
        for extra in &config.extra_folders {
            self.fs.create_dir_all(&pkg_path.join(extra))?;
        }

        Ok(())
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
                        errors.push(format!("Hash Mismatch: {path:?}"));
                    }
                },
                Err(_) => errors.push(format!("File Missing: {path:?}")),
            }
        }

        if errors.is_empty() {
            Ok(start.elapsed())
        } else {
            Err(errors)
        }
    }

    fn generate_custom_tree(
        &self,
        manifest: &mut HashMap<PathBuf, String>,
        base: &Path,
        spec: &DirSpec,
    ) -> io::Result<()> {
        match spec {
            DirSpec::List(items) => {
                for item in items {
                    let dir = base.join(item);
                    self.create_module_skeleton(manifest, &dir)?;
                }
            },
            DirSpec::Tree(map) => {
                for (name, child) in map {
                    let dir = base.join(name);
                    self.create_module_skeleton(manifest, &dir)?;
                    self.generate_custom_tree(manifest, &dir, child)?;
                }
            },
        }

        Ok(())
    }

    fn create_module_skeleton(
        &self,
        manifest: &mut HashMap<PathBuf, String>,
        dir: &Path,
    ) -> io::Result<()> {
        let tests_mod = dir.join("tests/mod.rs");
        let unit_mod = dir.join("tests/unit/mod.rs");
        let integration_mod = dir.join("tests/integration/mod.rs");

        // Create src + tests
        self.fs.create_dir_all(unit_mod.parent().unwrap())?;
        self.fs.create_dir_all(integration_mod.parent().unwrap())?;

        // mod.rs
        self.fs
            .write_file(&dir.join("mod.rs"), "pub mod tests;\n")?;

        // tests
        self.fs.write_file(&tests_mod, "// Tests\n")?;
        self.fs.write_file(&unit_mod, "// Unit Tests\n")?;
        self.fs
            .write_file(&integration_mod, "// Integration Tests\n")?;

        manifest.insert(tests_mod, self.calculate_hash("// Tests\n"));
        manifest.insert(unit_mod, self.calculate_hash("// Unit Tests\n"));
        manifest.insert(
            integration_mod,
            self.calculate_hash("// Integration Tests\n"),
        );

        Ok(())
    }

    fn resolve_template_name(&self, name: &str) -> Option<String> {
        // 1. Exact match
        if self.env.get_template(name).is_ok() {
            return Some(name.to_string());
        }

        // 2. Strip leading "templates/"
        if let Some(stripped) = name.strip_prefix("templates/")
            && self.env.get_template(stripped).is_ok()
        {
            return Some(stripped.to_string());
        }

        // 3. Try prefixing "readme/"
        let prefixed = format!("readme/{name}");
        if self.env.get_template(&prefixed).is_ok() {
            return Some(prefixed);
        }

        // 4. Try searching by filename only
        let filename = name.split('/').next_back().unwrap();
        for candidate in Asset::iter() {
            if candidate.ends_with(filename) {
                return Some(candidate.to_string());
            }
        }

        None
    }

    fn render_path(&self, raw: &str, ctx: &minijinja::value::Value) -> String {
        self.env
            .render_str(raw, ctx)
            .expect("Failed to render path template")
    }
}
