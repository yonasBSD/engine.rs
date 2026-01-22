use crate::builder::ConfigBuilder;

/// Fluent DSL for building config.toml in tests.
pub trait ConfigBuilderDsl {
    fn project(self, name: &str) -> Self;
    fn feature(self, name: &str) -> Self;
    fn package(self, name: &str) -> Self;
    fn readme(self, file: &str, path: &str) -> Self;
    fn custom_module(self, path: &str, items: &[&str]) -> Self;
}

impl ConfigBuilderDsl for ConfigBuilder {
    fn project(mut self, name: &str) -> Self {
        self.projects.push(name.into());
        self
    }

    fn feature(mut self, name: &str) -> Self {
        self.features.push(name.into());
        self
    }

    fn package(mut self, name: &str) -> Self {
        self.packages.push(name.into());
        self
    }

    fn readme(mut self, file: &str, path: &str) -> Self {
        self.readmes.push((file.into(), path.into()));
        self
    }

    fn custom_module(mut self, path: &str, items: &[&str]) -> Self {
        self.custom_modules.push((
            path.into(),
            items.iter().map(std::string::ToString::to_string).collect(),
        ));
        self
    }
}
