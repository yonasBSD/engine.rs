#[macro_export]
macro_rules! dirs {
    // Top-level map: { "a" => ..., "b" => ... }
    ({ $($key:tt => $value:tt),* $(,)? }) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), dirs!($value));
        )*
        map
    }};

    // Nested map: { "frontends": [...], "backends": [...] }
    ({ $($key:tt : $value:tt),* $(,)? }) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key.to_string(), dirs!($value));
        )*
        DirSpec::Tree(map)
    }};

    // List: ["a", "b", "c"]
    ([ $($item:tt),* $(,)? ]) => {
        DirSpec::List(vec![$($item.to_string()),*])
    };
}
