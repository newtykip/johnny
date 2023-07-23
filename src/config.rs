#[cfg(dev)]
use dotenvy_macro::dotenv as env;
use johnny::preludes::general::*;
use serde::{Deserialize, Serialize};
#[cfg(not(dev))]
use std::fs::read_to_string;

#[cfg(db)]
#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[cfg(johnny)]
#[derive(Serialize, Deserialize)]
pub struct JohnnyConfig {
    /// imgur client id
    pub imgur: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,

    #[cfg(db)]
    pub database: DatabaseConfig,

    #[cfg(johnny)]
    pub johnny: JohnnyConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        cfg_if! {
            if #[cfg(dev)] {
                Ok(Self {
                    token: env!("DISCORD_TOKEN").into(),
                    #[cfg(db)]
                    database: DatabaseConfig {
                        url: env!("DATABASE_URL").into(),
                    },
                    #[cfg(johnny)]
                    johnny: JohnnyConfig {
                        imgur: env!("IMGUR_CLIENT_ID").into(),
                    },
                })
            } else {
                toml::from_str::<Config>(
                    read_to_string("config.toml")
                        .wrap_err("config.toml should exist")
                        .suggestion("create a config.toml file, you can find an example at https://github.com/newtykins/johnny/blob/main/config.toml.example")?
                        .as_str(),
                )
                .wrap_err("config.toml should be valid toml")
            }
        }
    }
}
