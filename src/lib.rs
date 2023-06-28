mod control_panel;
pub mod logger;

pub use control_panel::run_tui;
use debug_ignore::DebugIgnore;
use poise::{
    serenity_prelude::{ChannelId, EmojiId, Member, User},
    CreateReply,
};
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
pub const JOHNNY_GALLERY_ID: &str = "qsKCczF";

// channel ids
pub const SUGGESTIONS_ID: ChannelId = ChannelId(1120764782014890032);

// emoji ids
pub const UPVOTE_ID: EmojiId = EmojiId(1120764904656351324);
pub const DOWNVOTE_ID: EmojiId = EmojiId(1120764921555206336);

/// Set the author of an embed to the author of the message
pub async fn create_embed(user: &User, member: Option<Member>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let mut name = user.name.clone();
    let mut avatar_option = user.avatar_url();

    if let Some(member) = member {
        // if the author is a member, use their display name and avatar
        name = member.display_name().to_string();
        avatar_option = member.avatar_url()
    }

    // if the avatar is none, use the default
    let avatar = avatar_option.unwrap_or_else(|| user.default_avatar_url());

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
        .unwrap()
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
    pub log: mpsc::Sender<logger::Entry>,
}

#[derive(Debug)]
pub struct Bot {
    pub senders: DebugIgnore<BotSenders>,
}

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
