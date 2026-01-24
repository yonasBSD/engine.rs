use std::{fs, path::Path};

pub fn read_file(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("failed to read file")
}
