#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use migration::TABLES;
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use sea_orm_codegen::{DateTimeCrate, EntityTransformer, EntityWriterContext, WithSerde};
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generate entity files
    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    let files = EntityTransformer::transform(TABLES.clone())?
        .generate(&EntityWriterContext::new(
            false,
            WithSerde::None,
            false,
            DateTimeCrate::Time,
            Some("public".into()),
            false,
            false,
            false,
            vec![],
            vec![],
        ))
        .files;

    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    for out_file in files {
        let mut file = File::create(
            PathBuf::new()
                .join("src")
                .join("db")
                .join("entity")
                .join(&out_file.name),
        )?;

        let contents = if out_file.name != "mod.rs" {
            // pass the file to rustfmt
            let proc = Command::new("rustfmt")
                .args(["/dev/fd/0", "--emit", "stdout", "--edition", "2021"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            proc.stdin
                .as_ref()
                .expect("it always exists")
                .write_all(out_file.content.as_bytes())?;

            let output = proc.wait_with_output()?;
            let output = String::from_utf8_lossy(output.stdout.as_slice());
            let pos = output.find("\n\n").unwrap() + 2;

            output[pos..].to_string()
        } else {
            out_file.content.replace(" ;", ";")
        };

        file.write_all(contents.as_bytes())?;
    }

    cfg_aliases! {
        // general

        // is the tui enabled?
        tui = feature = "tui",
        // is the bot johnny?
        johnny = feature = "johnny",
        // should the logger be verbose?
        verbose = feature = "verbose",

        // database drivers

        // does the bot use sqlite?
        sqlite = feature = "sqlite",
        // is a single database driver enabled?
        db = any(feature = "postgres", feature = "mysql", feature = "sqlite"),
        // are multiple of the database drivers enabled?
        multiple_db = any(all(feature = "postgres", feature = "mysql"), all(feature = "mysql", feature = "sqlite"), all(feature = "postgres", feature = "sqlite"), all(feature = "postgres", feature = "mysql", feature = "sqlite")),

        // modules

        // is autorole enabled?
        autorole = feature = "autorole",

        // is image enabled?
        image = feature = "image"
    }

    Ok(())
}
