#[cfg(db)]
pub mod db;
pub mod embed;
pub mod logger;
mod macros;
pub mod preludes;

use logger::Logger;
#[cfg(johnny)]
use poise::serenity_prelude::{ChannelId, EmojiId, ReactionType};
#[cfg(db)]
use poise::serenity_prelude::{GuildId, UserId};
use poise::serenity_prelude::{Member, User};
use preludes::general::*;
#[cfg(johnny)]
use rand::Rng;
#[cfg(db)]
use sea_orm::DatabaseConnection;
use std::borrow::Cow;
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};

pub struct Data {
    #[cfg(johnny)]
    pub johnny_images: Vec<String>,
    #[cfg(db)]
    pub db: DatabaseConnection,
    pub logger: Logger,
    #[cfg(db)]
    pub guilds_in_db: RwLock<HashSet<GuildId>>,
    #[cfg(db)]
    pub users_in_db: RwLock<HashSet<UserId>>,
    #[cfg(db)]
    pub members_in_db: RwLock<HashSet<(GuildId, UserId)>>,
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub const EPOCH: i64 = 1420070400000;

// channel ids
#[cfg(johnny)]
pub const SUGGESTIONS_ID: ChannelId = ChannelId(1120764782014890032);

// reactions
#[cfg(johnny)]
pub struct Reactions {
    pub upvote: ReactionType,
    pub downvote: ReactionType,
}

#[cfg(johnny)]
impl Default for Reactions {
    fn default() -> Self {
        Self {
            upvote: ReactionType::Custom {
                animated: false,
                id: EmojiId(1120764904656351324),
                name: Some("upvote".into()),
            },
            downvote: ReactionType::Custom {
                animated: false,
                id: EmojiId(1120764921555206336),
                name: Some("downvote".into()),
            },
        }
    }
}

pub fn determine_avatar(user: &User, member: Option<Cow<'_, Member>>) -> String {
    member
        .and_then(|x| x.avatar_url())
        .or(user.avatar_url())
        .unwrap_or(user.default_avatar_url())
}

#[cfg(johnny)]
pub const JOHNNY_GALLERY_IDS: [&str; 2] = ["oPluI3u", "Ca2YQ2O"];

/// Get a random johnny image
#[cfg(johnny)]
pub fn johnny_image(data: &Data) -> (usize, String) {
    let index = rand::thread_rng().gen_range(0..data.johnny_images.len());

    (index + 1, data.johnny_images[index].clone())
}
