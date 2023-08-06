#[path = "preludes/command.rs"]
pub mod command;
#[cfg(db)]
pub mod db;
pub mod embed;
#[path = "preludes/event.rs"]
pub mod event;
mod macros;
pub mod prelude;

use color_eyre::eyre::Error;
use poise::serenity_prelude::{Member, User};
use std::borrow::Cow;
#[cfg(db)]
use {
    db::DB,
    sqlx::{Database, Pool},
    std::collections::HashSet,
};

/// command context
#[cfg(db)]
pub type Context<'a> = poise::Context<'a, Data<DB>, Error>;
#[cfg(not(db))]
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// poise framework data
pub struct Data<#[cfg(db)] D: Database = DB> {
    #[cfg(johnny)]
    pub johnny_images: Vec<String>,
    #[cfg(db)]
    pub pool: Pool<D>,
    #[cfg(db)]
    pub user_cache: HashSet<u64>,
    /// guild id, user id
    #[cfg(db)]
    pub member_cache: HashSet<(u64, u64)>,
}

/// bot epoch timestamp
pub const EPOCH: i64 = 1420070400000;

pub fn determine_avatar(user: &User, member: Option<Cow<'_, Member>>) -> String {
    member
        .and_then(|x| x.avatar_url())
        .or(user.avatar_url())
        .unwrap_or(user.default_avatar_url())
}
