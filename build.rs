/// Create an alias for `#[cfg]` attributes to use
macro_rules! cfg_aliases {
    ($($alias:tt = $config:meta),* $(,)*) => {
        $(
            if cfg!($config) {
                println!("cargo:rustc-cfg={}", stringify!($alias));
            }
        )*
    };
}

fn main() {
    cfg_aliases! {
        // is a single database driver enabled?
        db = any(feature = "mysql", feature = "postgres", feature = "sqlite"),
        // are multiple of the database drivers enabled?
        multiple_db = any(all(feature = "mysql", feature = "postgres"), all(feature = "mysql", feature = "sqlite"), all(feature = "postgres", feature = "sqlite"), all(feature = "mysql", feature = "postgres", feature = "sqlite")),
    }
}
