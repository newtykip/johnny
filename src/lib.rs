pub mod logger;

#[cfg(feature = "johnny")]
use once_cell::sync::Lazy;
use poise::{
    serenity_prelude::{ChannelId, EmojiId, ReactionType},
    CreateReply,
};
#[cfg(feature = "johnny")]
use rand::seq::SliceRandom;
use serenity::{builder::CreateEmbed, utils::Colour};
use tokio::sync::mpsc;

pub struct Data {
    #[cfg(feature = "johnny")]
    pub johnny_images: Vec<String>,
    pub logger: logger::Logger,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// The id of the johnny gallery on imgur
// todo: make this the correct id when i get the images
#[cfg(feature = "johnny")]
pub const JOHNNY_GALLERY_ID: &str = "qsKCczF";

// channel ids
#[cfg(feature = "johnny")]
pub const SUGGESTIONS_ID: ChannelId = ChannelId(1120764782014890032);

// emoji ids
#[cfg(feature = "johnny")]
pub const UPVOTE_REACTION: Lazy<ReactionType> = Lazy::new(|| ReactionType::Custom {
    animated: false,
    id: EmojiId(1120764904656351324),
    name: Some("upvote".into()),
});
#[cfg(feature = "johnny")]
pub const DOWNVOTE_REACTION: Lazy<ReactionType> = Lazy::new(|| ReactionType::Custom {
    animated: false,
    id: EmojiId(1120764921555206336),
    name: Some("downvote".into()),
});

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
                senders: BotSenders { log: log_tx }.into(),
            },
            BotRecievers { log: log_rx },
        )
    }
}
