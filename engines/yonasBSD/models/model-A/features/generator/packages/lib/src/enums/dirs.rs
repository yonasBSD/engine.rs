use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
