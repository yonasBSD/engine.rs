#[derive(Default)]
pub struct ConfigBuilder {
    pub projects: Vec<String>,
    pub features: Vec<String>,
    pub packages: Vec<String>,
    pub readmes: Vec<(String, String)>,
    pub custom_modules: Vec<(String, Vec<String>)>,
}

impl ConfigBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> String {
        let mut out = String::new();

        if !self.projects.is_empty() {
            out += &format!("projects = {:?}\n", self.projects);
        }
        if !self.features.is_empty() {
            out += &format!("features = {:?}\n", self.features);
        }
        if !self.packages.is_empty() {
            out += &format!("packages = {:?}\n", self.packages);
        }

        for (file, path) in self.readmes {
            out += &format!("\n[[readme]]\nfile = {file:?}\npath = {path:?}\n");
        }

        for (module_path, items) in self.custom_modules {
            out += &format!("\n[custom_modules.{module_path}]\nbackends = {items:?}\n");
        }

        out
    }
}
