use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DirSpec {
    List(Vec<String>),
    Tree(HashMap<String, Self>),
}

impl Default for DirSpec {
    fn default() -> Self {
        Self::List(vec![])
    }
}
