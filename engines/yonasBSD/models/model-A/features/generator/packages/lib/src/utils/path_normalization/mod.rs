use std::path::{Path, PathBuf};

/// Normalize a path string:
/// - convert backslashes to slashes
/// - collapse duplicate slashes
/// - remove trailing slash
#[must_use]
pub fn normalize_path_str(p: &str) -> String {
    p.replace('\\', "/")
        .replace("//", "/")
        .trim_end_matches('/')
        .to_string()
}

/// Normalize a Path into a canonicalized `PathBuf` without touching the
/// filesystem.
#[must_use]
pub fn normalize_path(p: &Path) -> PathBuf {
    PathBuf::from(normalize_path_str(&p.to_string_lossy()))
}
