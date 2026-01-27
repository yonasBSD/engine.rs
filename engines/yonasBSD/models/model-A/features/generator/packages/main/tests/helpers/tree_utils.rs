use std::path::Path;

use walkdir::WalkDir;

pub fn capture_tree(root: impl AsRef<Path>) -> String {
    let root = root.as_ref();

    let mut entries: Vec<(String, bool)> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let path = entry.path().strip_prefix(root).ok()?;
            let s = path.display().to_string();

            // Skip the root entry (empty string)
            if s.is_empty() {
                return None;
            }

            let is_dir = entry.file_type().is_dir();
            Some((s, is_dir))
        })
        .collect();

    // Sort: directories first, then files; alphabetical within each group
    entries.sort_by(|(a_path, a_is_dir), (b_path, b_is_dir)| {
        b_is_dir.cmp(a_is_dir).then(a_path.cmp(b_path))
    });

    // Emit only the path strings
    let lines: Vec<String> = entries.into_iter().map(|(p, _)| p).collect();

    lines.join("\n") + "\n"
}
