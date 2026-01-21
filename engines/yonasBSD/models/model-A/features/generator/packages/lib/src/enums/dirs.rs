use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub enum DirSpec {
    List(Vec<String>),
    Tree(HashMap<String, DirSpec>),
}

impl Default for DirSpec {
    fn default() -> Self {
        DirSpec::List(vec![])
    }
}
