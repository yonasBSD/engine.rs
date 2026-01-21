use std::path::Path;
use walkdir::WalkDir;

pub fn capture_tree(root: impl AsRef<Path>) -> String {
    let root = root.as_ref();
    let mut out = String::new();

    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        let path = entry.path().strip_prefix(root).unwrap();
        out.push_str(&format!("{}\n", path.display()));
    }

    out
}
