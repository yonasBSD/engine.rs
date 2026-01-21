use std::path::Path;
use walkdir::WalkDir;

/// Capture a directory tree as a stable, sorted string.
/// This is perfect for snapshot testing.
pub fn capture_tree(root: &Path) -> String {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap();

        if rel.as_os_str().is_empty() {
            continue;
        }

        if path.is_dir() {
            entries.push(format!("{}/", rel.display()));
        } else {
            entries.push(format!("{}", rel.display()));
        }
    }

    entries.join("\n")
}

pub fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| "<missing>".into())
}
