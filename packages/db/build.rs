use cfg_aliases::cfg_aliases;
use migration::TABLES;
use rust_format::{Formatter, RustFmt};
use sea_orm_codegen::{DateTimeCrate, EntityTransformer, EntityWriterContext, WithSerde};
use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    // generate entity files
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

    for out_file in files {
        let mut file = File::create(format!("src/entity/{}", out_file.name))?;
        let formatted = RustFmt::default().format_str(out_file.content)?;
        file.write_all(formatted.as_bytes())?;
    }

    cfg_aliases! {
        autorole: { feature = "autorole" },
        sticky: { feature = "sticky" },
        events: { feature = "events" }
    }

    Ok(())
}
