use common::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[cfg(db)]
#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,

    #[cfg(db)]
    pub database: DatabaseConfig,

    #[cfg(johnny)]
    pub johnny: johnny::Config,
}

impl Config {
    pub fn load() -> Result<Self> {
        toml::from_str::<Config>(
            read_to_string("config.toml")
                .wrap_err("config.toml should exist")
                .suggestion("create a config.toml file, you can find an example at https://github.com/newtykins/johnny/blob/main/config.toml.example")?
                .as_str(),
        )
        .wrap_err("config.toml should be valid toml")
    }
}
