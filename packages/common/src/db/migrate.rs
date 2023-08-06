#[path = "migrations/mod.rs"]
mod migrations;

use crate::prelude::*;
use refinery::{config::Config, Report};
use std::str::FromStr;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("src/db/migrations");
}

pub fn migrate(url: String) -> Result<Report> {
    let mut conn = Config::from_str(&url)?;

    embedded::migrations::runner()
        .run(&mut conn)
        .wrap_err("failed to run migrations")
}
