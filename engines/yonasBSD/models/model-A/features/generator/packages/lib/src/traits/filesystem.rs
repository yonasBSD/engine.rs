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
        Asset, Config, DEFAULT_README_TPL, EXTRA_TOP_LEVEL_DIRS, TPL_CARGO, TPL_MEMBER_CARGO,
        TPL_MOD_EXPORT, TPL_MOD_TESTS, internal::DEFAULT_PACKAGES,
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
// TEMPLATE STORE — avoids Box::leak by owning strings
// ==========================================================
struct TemplateStore {
    entries: Vec<(String, String)>,
}

impl TemplateStore {
    fn new() -> Self {
        let mut entries = Vec::new();

        for file in Asset::iter() {
            if let Some(content) = Asset::get(&file) {
                if let Ok(body) = std::str::from_utf8(content.data.as_ref()) {
                    entries.push((file.to_string(), body.to_owned()));
                }
            }
        }

        // Internal fallbacks (embedded assets registered first take priority;
        // these are no-ops if the asset already exists under the same name).
        entries.push(("internal/readme.tpl".into(), DEFAULT_README_TPL.into()));
        entries.push(("internal/cargo.tpl".into(), TPL_CARGO.into()));
        entries.push(("internal/member_cargo.tpl".into(), TPL_MEMBER_CARGO.into()));
        entries.push(("internal/mod_export.tpl".into(), TPL_MOD_EXPORT.into()));
        entries.push(("internal/mod_tests.tpl".into(), TPL_MOD_TESTS.into()));

        Self {
            entries,
        }
    }

    /// Load all stored templates into a minijinja Environment.
    ///
    /// Uses `Box::leak` intentionally and only once per call, keeping the
    /// leak surface minimal and clearly isolated here rather than scattered
    /// through business logic.
    fn build_env(&self) -> Environment<'static> {
        let mut env = Environment::new();
        for (name, body) in &self.entries {
            let name: &'static str = Box::leak(name.clone().into_boxed_str());
            let body: &'static str = Box::leak(body.clone().into_boxed_str());
            // Silently skip duplicates — embedded assets take priority.
            let _ = env.add_template(name, body);
        }
        env
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
        let env = TemplateStore::new().build_env();
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

        // Cartesian product: projects × features × packages
        for project in &config.projects {
            let project_root = self.base_path.join(format!("engines/{project}"));

            for feature in &config.features {
                let feature_root = project_root.join(format!("models/model-A/features/{feature}"));
                let packages_root = feature_root.join("packages");

                // 1. Generate workspace Cargo.toml for this feature
                self.write_workspace_cargo(config, &feature_root, &mut manifest)?;

                // 2. Generate default sub-packages
                for pkg in DEFAULT_PACKAGES {
                    let pkg_path = packages_root.join(pkg);
                    self.generate_package_structure(
                        &mut manifest,
                        config,
                        project,
                        feature,
                        pkg,
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

    // ----------------------------------------------------------
    // Workspace Cargo.toml
    // ----------------------------------------------------------
    fn write_workspace_cargo(
        &self,
        config: &Config,
        feature_root: &Path,
        manifest: &mut HashMap<PathBuf, String>,
    ) -> io::Result<()> {
        // Build the members list from DEFAULT_PACKAGES + user-defined packages.
        // DEFAULT_PACKAGES is the single source of truth — no hardcoding here.
        let members: String = DEFAULT_PACKAGES
            .iter()
            .map(|p| format!("    \"packages/{p}\","))
            .chain(
                config
                    .packages
                    .iter()
                    .map(|p| format!("    \"packages/{p}\",")),
            )
            .collect::<Vec<_>>()
            .join("\n");

        let ctx = context!(
            members   => members,
            workspace => &config.workspace,
        );

        let cargo_content = self
            .render_template("internal/cargo.tpl", &ctx)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let cargo_path = feature_root.join("Cargo.toml");
        self.fs.create_dir_all(cargo_path.parent().unwrap())?;
        self.fs.write_file(&cargo_path, &cargo_content)?;
        manifest.insert(cargo_path, self.calculate_hash(&cargo_content));

        Ok(())
    }

    // ----------------------------------------------------------
    // Per-package orchestration
    // ----------------------------------------------------------
    fn generate_package_structure(
        &self,
        manifest: &mut HashMap<PathBuf, String>,
        config: &Config,
        project: &str,
        feature: &str,
        package: &str,
        pkg_path: &Path,
    ) -> io::Result<()> {
        let ctx = context!(
            project => project,
            feature => feature,
            package => package,
            model   => "model-A",
        );

        self.write_readmes(config, &ctx, manifest)?;
        self.write_member_cargo(&ctx, pkg_path, manifest)?;
        self.write_mirrored_modules(&ctx, pkg_path, manifest)?;
        self.write_extra_dirs(config, pkg_path)?;

        if let Some(spec) = config.custom_modules.get(package) {
            self.generate_custom_tree(manifest, pkg_path, spec)?;
        }

        Ok(())
    }

    // ----------------------------------------------------------
    // README generation
    // ----------------------------------------------------------
    fn write_readmes(
        &self,
        config: &Config,
        ctx: &minijinja::value::Value,
        manifest: &mut HashMap<PathBuf, String>,
    ) -> io::Result<()> {
        for entry in &config.readmes {
            let tpl_name = self
                .resolve_template_name(&entry.file)
                .unwrap_or_else(|| "internal/readme.tpl".to_string());

            let rendered = self
                .render_template(&tpl_name, ctx)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let rendered_path = self.render_path(&entry.path, ctx);
            let normalized: PathBuf = PathBuf::from(rendered_path).components().collect();
            let dest = self.base_path.join(normalized).join("README.md");

            self.fs.create_dir_all(dest.parent().unwrap())?;
            self.fs.write_file(&dest, &rendered)?;
            manifest.insert(dest, self.calculate_hash(&rendered));
        }

        Ok(())
    }

    // ----------------------------------------------------------
    // Member Cargo.toml + src/lib.rs
    // The template itself handles the conditional `lib` dep via
    // `{% if package != "lib" %}`.
    // ----------------------------------------------------------
    fn write_member_cargo(
        &self,
        ctx: &minijinja::value::Value,
        pkg_path: &Path,
        manifest: &mut HashMap<PathBuf, String>,
    ) -> io::Result<()> {
        let member_cargo_content = self
            .render_template("internal/member_cargo.tpl", ctx)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let member_cargo_path = pkg_path.join("Cargo.toml");
        self.fs.create_dir_all(pkg_path)?;
        self.fs
            .write_file(&member_cargo_path, &member_cargo_content)?;
        manifest.insert(
            member_cargo_path,
            self.calculate_hash(&member_cargo_content),
        );

        let src_dir = pkg_path.join("src");
        self.fs.create_dir_all(&src_dir)?;

        let mod_content = self
            .render_template("internal/mod_export.tpl", ctx)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mod_path = pkg_path.join("src/lib.rs");
        self.fs.write_file(&mod_path, &mod_content)?;
        manifest.insert(mod_path, self.calculate_hash(&mod_content));

        Ok(())
    }

    // ----------------------------------------------------------
    // Mirrored modules (src/{mod}/mod.rs + test scaffolding)
    // ----------------------------------------------------------
    fn write_mirrored_modules(
        &self,
        ctx: &minijinja::value::Value,
        pkg_path: &Path,
        manifest: &mut HashMap<PathBuf, String>,
    ) -> io::Result<()> {
        const MODULES: &[&str] = &[
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

        let t_content = self
            .render_template("internal/mod_tests.tpl", ctx)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        for mod_dir in MODULES {
            let base = pkg_path.join(format!("src/{mod_dir}"));
            self.write_module_with_tests(&base, &t_content, manifest)?;
        }

        // core/mod.rs re-exports its sub-modules
        let core_mod_path = pkg_path.join("src/core/mod.rs");
        let core_mod_content = r#"pub mod backends;
pub mod frontends;
pub mod public;
pub mod internal;
pub mod private;

#[allow(unused_imports)]
pub use public::*;"#;
        self.fs.write_file(&core_mod_path, core_mod_content)?;
        manifest.insert(core_mod_path, self.calculate_hash(core_mod_content));

        // Extra top-level dirs
        for extra in EXTRA_TOP_LEVEL_DIRS {
            self.fs.create_dir_all(&pkg_path.join(extra))?;
        }

        Ok(())
    }

    /// Write mod.rs + tests/{unit,integration}/mod.rs for a single module
    /// directory.
    fn write_module_with_tests(
        &self,
        base: &Path,
        tests_content: &str,
        manifest: &mut HashMap<PathBuf, String>,
    ) -> io::Result<()> {
        let tests_mod = base.join("tests/mod.rs");
        let unit_mod = base.join("tests/unit/mod.rs");
        let integration_mod = base.join("tests/integration/mod.rs");

        self.fs.create_dir_all(unit_mod.parent().unwrap())?;
        self.fs.create_dir_all(integration_mod.parent().unwrap())?;

        self.fs.write_file(&base.join("mod.rs"), "mod tests;\n")?;
        self.fs.write_file(&tests_mod, tests_content)?;
        self.fs.write_file(&unit_mod, "// Automated Unit Tests\n")?;
        self.fs
            .write_file(&integration_mod, "// Automated Integration Tests\n")?;

        manifest.insert(tests_mod, self.calculate_hash(tests_content));
        manifest.insert(unit_mod, self.calculate_hash("// Automated Unit Tests\n"));
        manifest.insert(
            integration_mod,
            self.calculate_hash("// Automated Integration Tests\n"),
        );

        Ok(())
    }

    // ----------------------------------------------------------
    // Extra dirs
    // ----------------------------------------------------------
    fn write_extra_dirs(&self, config: &Config, pkg_path: &Path) -> io::Result<()> {
        for extra in &config.extra_folders {
            self.fs.create_dir_all(&pkg_path.join(extra))?;
        }
        Ok(())
    }

    // ----------------------------------------------------------
    // Integrity check
    // ----------------------------------------------------------
    pub fn verify_integrity(
        &self,
        manifest: &HashMap<PathBuf, String>,
    ) -> Result<Duration, Vec<String>> {
        let start = Instant::now();
        let mut errors = Vec::new();

        for (path, expected_hash) in manifest {
            match self.fs.read_to_string(path) {
                Ok(content) => {
                    if self.calculate_hash(&content) != *expected_hash {
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

    // ----------------------------------------------------------
    // Custom tree (DSL)
    // ----------------------------------------------------------
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
        const TESTS_CONTENT: &str = "// Tests\n";
        const UNIT_CONTENT: &str = "// Unit Tests\n";
        const INTEGRATION_CONTENT: &str = "// Integration Tests\n";

        let tests_mod = dir.join("tests/mod.rs");
        let unit_mod = dir.join("tests/unit/mod.rs");
        let integration_mod = dir.join("tests/integration/mod.rs");

        self.fs.create_dir_all(unit_mod.parent().unwrap())?;
        self.fs.create_dir_all(integration_mod.parent().unwrap())?;

        self.fs
            .write_file(&dir.join("mod.rs"), "pub mod tests;\n")?;
        self.fs.write_file(&tests_mod, TESTS_CONTENT)?;
        self.fs.write_file(&unit_mod, UNIT_CONTENT)?;
        self.fs.write_file(&integration_mod, INTEGRATION_CONTENT)?;

        manifest.insert(tests_mod, self.calculate_hash(TESTS_CONTENT));
        manifest.insert(unit_mod, self.calculate_hash(UNIT_CONTENT));
        manifest.insert(integration_mod, self.calculate_hash(INTEGRATION_CONTENT));

        Ok(())
    }

    // ----------------------------------------------------------
    // Template helpers
    // ----------------------------------------------------------
    fn render_template(
        &self,
        name: &str,
        ctx: &minijinja::value::Value,
    ) -> Result<String, minijinja::Error> {
        self.env.get_template(name)?.render(ctx.clone())
    }

    fn resolve_template_name(&self, name: &str) -> Option<String> {
        // 1. Exact match
        if self.env.get_template(name).is_ok() {
            return Some(name.to_string());
        }

        // 2. Strip leading "templates/"
        if let Some(stripped) = name.strip_prefix("templates/") {
            if self.env.get_template(stripped).is_ok() {
                return Some(stripped.to_string());
            }
        }

        // 3. Try prefixing "readme/"
        let prefixed = format!("readme/{name}");
        if self.env.get_template(&prefixed).is_ok() {
            return Some(prefixed);
        }

        // 4. Search by filename only
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
