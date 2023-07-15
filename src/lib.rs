#[cfg(db)]
pub mod db;
pub mod logger;

#[cfg(johnny)]
use poise::serenity_prelude::{ChannelId, EmojiId, ReactionType};
#[cfg(db)]
use poise::serenity_prelude::{GuildId, UserId};
use poise::serenity_prelude::{Member, User};
#[cfg(johnny)]
use rand::seq::SliceRandom;
#[cfg(db)]
use sea_orm::DatabaseConnection;
use serenity::{builder::CreateEmbed, utils::Colour};
use std::borrow::Cow;
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};

pub struct Data {
    #[cfg(johnny)]
    pub johnny_images: Vec<String>,
    #[cfg(db)]
    pub db: DatabaseConnection,
    pub logger: logger::Logger,
    #[cfg(db)]
    pub guilds_in_db: RwLock<HashSet<GuildId>>,
    #[cfg(db)]
    pub users_in_db: RwLock<HashSet<UserId>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

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

const EMBED_COLOUR: Colour = Colour::from_rgb(192, 238, 255);

/// Generate the base of any embed
pub fn generate_base_embed(user: &User, member: Option<Cow<'_, Member>>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let name = match &member {
        Some(member) => member.display_name().to_string(),
        None => user.name.clone(),
    };

    let avatar = match member {
        Some(member) => member.avatar_url(),
        None => user.avatar_url(),
    }
    .unwrap_or(user.default_avatar_url());

    embed
        .author(|author| author.name(name).icon_url(avatar))
        .color(EMBED_COLOUR)
        .clone()
}

#[cfg(johnny)]
pub const JOHNNY_GALLERY_IDS: [&str; 2] = ["oPluI3u", "Ca2YQ2O"];

/// Get a random johnny image
#[cfg(johnny)]
pub fn johnny_image(data: &Data) -> String {
    data.johnny_images
        .choose(&mut rand::thread_rng())
        .expect("there should be images of johnny loaded into the bot's data")
        .clone()
}
