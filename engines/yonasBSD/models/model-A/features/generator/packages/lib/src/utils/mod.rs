pub mod tests;
pub mod path_normalization;

#[must_use]
pub fn default_projects() -> Vec<String> {
    vec!["yonasBSD".to_string()]
}

#[must_use]
pub fn default_features() -> Vec<String> {
    vec!["feature-A".to_string()]
}

#[must_use]
pub fn default_packages() -> Vec<String> {
    vec!["package-A".to_string()]
}
