#[macro_export]
macro_rules! engine {
    (
        projects = [$($proj:expr),*],
        features = [$($feat:expr),*],
        packages = [$($pkg:expr),*],
        $($rest:tt)*
    ) => {{
        let mut b = engine_rs_lib::ConfigBuilder::new();
        $( b = b.project($proj); )*
        $( b = b.feature($feat); )*
        $( b = b.package($pkg); )*
        engine!(@inner b, $($rest)*)
    }};

    (@inner $b:expr,) => { $b };

    (@inner $b:expr, readme($file:expr, $path:expr), $($rest:tt)*) => {{
        let b = $b.readme($file, $path);
        engine!(@inner b, $($rest)*)
    }};

    (@inner $b:expr, readme($file:expr, $path:expr)) => {{
        $b.readme($file, $path)
    }};
}
