#[derive(Debug)]
pub enum ConfigError {
    EmptyList {
        field: &'static str,
    },
    DuplicateValues {
        field: &'static str,
        duplicates: Vec<String>,
    },
    InvalidName {
        field: &'static str,
        value: String,
    },
}
