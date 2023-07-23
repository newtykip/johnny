use poise::serenity_prelude::{Guild, Member, User};
use serenity::{builder::CreateEmbed, utils::Colour};
use std::borrow::Cow;

use crate::determine_avatar;

pub mod colours {
    use serenity::utils::Colour;

    pub const DEFAULT: Colour = Colour::from_rgb(192, 238, 255);
    pub const SUCCESS: Colour = Colour::from_rgb(95, 252, 198);
    pub const FAILURE: Colour = Colour::from_rgb(255, 171, 171);
}

/// Generate the base of any embed
pub fn generate_embed(
    user: &User,
    member: Option<Cow<'_, Member>>,
    colour: Option<Colour>,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let name = match &member {
        Some(member) => member.display_name().to_string(),
        None => user.name.clone(),
    };

    let avatar = determine_avatar(user, member);

    embed
        .author(|author| author.name(name).icon_url(avatar))
        .color(colour.unwrap_or(colours::DEFAULT))
        .clone()
}

pub fn set_guild_thumbnail(embed: &mut CreateEmbed, guild: Guild) {
    if let Some(url) = guild.icon_url() {
        embed.thumbnail(url);
    }
}
