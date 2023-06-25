use poise::serenity_prelude::{ChannelId, EmojiId};
use serenity::builder::CreateEmbed;

pub struct Data {
    pub johnny_images: Vec<String>,
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
pub async fn set_embed_author(ctx: &Context<'_>, embed: &mut CreateEmbed) {
    let name: String;

    let created_by = ctx.author();
    let avatar_option: Option<String>;

    if let Some(member) = ctx.author_member().await {
        // if the author is a member, use their display name and avatar
        name = member.display_name().to_string();
        avatar_option = member.avatar_url()
    } else {
        // otherwise, use their username and default avatar
        name = created_by.name.clone();
        avatar_option = created_by.avatar_url()
    };

    // if the avatar is none, use the default
    let avatar = avatar_option
        .unwrap_or_else(|| created_by.default_avatar_url())
        .to_string();

    embed.author(|author| author.name(name).icon_url(avatar));
}
