#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashMap;

    fn cfg(p: Vec<&str>, f: Vec<&str>, pk: Vec<&str>) -> Config {
        Config {
            projects: p.into_iter().map(|s| s.to_string()).collect(),
            features: f.into_iter().map(|s| s.to_string()).collect(),
            packages: pk.into_iter().map(|s| s.to_string()).collect(),
            readmes: vec![],
            custom_modules: HashMap::<String, DirSpec>::new(),
            extra_folders: vec![],
        }
    }

    #[test]
    fn rejects_empty_lists() {
        let c = cfg(vec![], vec!["f"], vec!["p"]);
        assert!(c.validate().is_err());

        let c = cfg(vec!["a"], vec![], vec!["p"]);
        assert!(c.validate().is_err());

        let c = cfg(vec!["a"], vec!["f"], vec![]);
        assert!(c.validate().is_err());
    }

    #[test]
    fn rejects_duplicates() {
        let c = cfg(vec!["a", "a"], vec!["f"], vec!["p"]);
        let err = c.validate().unwrap_err();
        assert!(err.iter().any(|e| matches!(
            e,
            ConfigError::DuplicateValues {
                field: "projects",
                ..
            }
        )));
    }

    #[test]
    fn rejects_invalid_names() {
        let c = cfg(vec!["bad name"], vec!["f"], vec!["p"]);
        let err = c.validate().unwrap_err();
        assert!(err.iter().any(|e| matches!(
            e,
            ConfigError::InvalidName {
                field: "projects",
                ..
            }
        )));
    }

    #[test]
    fn accepts_valid_config() {
        let c = cfg(vec!["proj"], vec!["feat"], vec!["pkg"]);
        assert!(c.validate().is_ok());
    }
}
