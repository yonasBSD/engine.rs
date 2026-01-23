use crate::{EngineError, enums::DirSpec};
use std::collections::HashMap;

/// Insert a custom module specification into the `custom_modules` map,
/// using a dotted path like `"api.core"` and a list of backends.
///
/// This mirrors the TOML shape:
/// [`custom_modules.api.core`]
/// backends = ["graphql", "grpc", "rest"]
pub fn insert_custom_module(
    custom_modules: &mut HashMap<String, DirSpec>,
    path: &str,
    backends: &[&str],
) -> Result<(), EngineError> {
    let parts: Vec<&str> = path.split('.').collect();

    // Reject empty segments
    if parts.iter().any(|p| p.trim().is_empty()) {
        //panic!("Invalid custom module path `{}`: empty segment", path);
        return Err(EngineError::invalid_path(path));
    }

    let (root, rest) = parts.split_first().unwrap();
    let spec = build_spec(rest, backends);

    custom_modules.insert((*root).to_string(), spec);
    Ok(())
}

fn build_spec(segments: &[&str], backends: &[&str]) -> DirSpec {
    // No nested segments: just a list of backends
    if segments.is_empty() {
        return DirSpec::List(
            backends
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        );
    }

    // Start from the leaf (backends list) and wrap upwards
    let mut current = DirSpec::List(
        backends
            .iter()
            .map(std::string::ToString::to_string)
            .collect(),
    );

    for seg in segments.iter().rev() {
        let mut map = HashMap::new();
        map.insert((*seg).to_string(), current);
        current = DirSpec::Tree(map);
    }

    current
}
