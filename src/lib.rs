#[cfg(db)]
pub mod db;
pub mod logger;

#[cfg(feature = "johnny")]
use poise::serenity_prelude::{ChannelId, EmojiId, ReactionType};
#[cfg(db)]
use poise::serenity_prelude::{GuildId, UserId};
use poise::CreateReply;
#[cfg(feature = "johnny")]
use rand::seq::SliceRandom;
#[cfg(db)]
use sea_orm::DatabaseConnection;
use serenity::{builder::CreateEmbed, utils::Colour};
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};
#[cfg(feature = "tui")]
use tokio::sync::mpsc;

pub struct Data {
    #[cfg(feature = "johnny")]
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
#[cfg(feature = "johnny")]
pub const SUGGESTIONS_ID: ChannelId = ChannelId(1120764782014890032);

// reactions
#[cfg(feature = "johnny")]
pub struct Reactions {
    pub upvote: ReactionType,
    pub downvote: ReactionType,
}

#[cfg(feature = "johnny")]
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

/// Set the author of an embed to the author of the message
pub async fn create_embed(ctx: &Context<'_>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    let user = ctx.author();
    let member = ctx.author_member().await;

    let mut name = user.name.clone();
    let mut avatar_option = user.avatar_url();

    if let Some(member) = member {
        // if the author is a member, use their display name and avatar
        name = member.display_name().to_string();
        avatar_option = member.avatar_url();
    }

    // determine the avatar
    let avatar = avatar_option
        .or(user.avatar_url())
        .or(Some(user.default_avatar_url()))
        .expect("there is definitely a default user avatar");

    embed
        .author(|author| author.name(name).icon_url(avatar))
        .colour(Colour::from_rgb(192, 238, 255))
        .clone()
}

#[cfg(feature = "johnny")]
pub const JOHNNY_GALLERY_IDS: [&str; 2] = ["oPluI3u", "Ca2YQ2O"];

/// Get a random johnny image
#[cfg(feature = "johnny")]
pub fn johnny_image(data: &Data) -> String {
    data.johnny_images
        .choose(&mut rand::thread_rng())
        .expect("there should be images of johnny loaded into the bot's data")
        .clone()
}

/// Add an embed to a message
pub fn apply_embed<'a, 'b>(
    msg: &'b mut CreateReply<'a>,
    embed: &CreateEmbed,
) -> &'b mut CreateReply<'a> {
    msg.embeds.push(embed.clone());
    msg
}

pub struct BotRecievers {
    pub log: logger::Reciever,
}

pub struct BotSenders {
    pub log: logger::Sender,
}

#[cfg(feature = "tui")]
pub struct Bot {
    pub senders: BotSenders,
}

#[cfg(feature = "tui")]
impl Bot {
    /// Initialise the state
    ///
    /// Returns the state, the online receiver, and the log receiver
    pub fn new() -> (Self, BotRecievers) {
        // start channels
        let (log_tx, log_rx) = mpsc::channel(32);

        (
            Self {
                senders: BotSenders { log: log_tx },
            },
            BotRecievers { log: log_rx },
        )
    }
}
