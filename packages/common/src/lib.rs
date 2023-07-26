pub mod embed;
mod macros;
pub mod prelude;
pub mod preludes;

use color_eyre::eyre::Error;
use poise::serenity_prelude::{Member, User};
#[cfg(db)]
use sea_orm::DatabaseConnection;
use std::borrow::Cow;

/// command context
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// poise framework data
pub struct Data {
    #[cfg(johnny)]
    pub johnny_images: Vec<String>,
    #[cfg(db)]
    pub db: DatabaseConnection,
}

/// bot epoch timestamp
pub const EPOCH: i64 = 1420070400000;

pub fn determine_avatar(user: &User, member: Option<Cow<'_, Member>>) -> String {
    member
        .and_then(|x| x.avatar_url())
        .or(user.avatar_url())
        .unwrap_or(user.default_avatar_url())
}
