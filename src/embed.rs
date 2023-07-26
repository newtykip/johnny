use poise::serenity_prelude::{Member, User};
use serenity::{builder::CreateEmbed, utils::Colour};
use std::borrow::Cow;

use crate::determine_avatar;

#[allow(non_upper_case_globals)]
pub mod colours {
    use serenity::utils::Colour;

    pub const Default: Colour = Colour::from_rgb(192, 238, 255);
    pub const Success: Colour = Colour::from_rgb(95, 252, 198);
    pub const Failure: Colour = Colour::from_rgb(255, 171, 171);
}

pub fn generate_embed(user: &User, member: Option<Cow<'_, Member>>, colour: Colour) -> CreateEmbed {
    let mut embed = CreateEmbed::default();

    let name = match &member {
        Some(member) => member.display_name().to_string(),
        None => user.name.clone(),
    };

    let avatar = determine_avatar(user, member);

    embed
        .author(|author| author.name(name).icon_url(avatar))
        .color(colour)
        .clone()
}

/// Generate the base of any embed
#[macro_export]
macro_rules! generate_embed {
    ($ctx: expr) => {
        $crate::embed::generate_embed(
            $ctx.author(),
            $ctx.author_member().await,
            $crate::embed::colours::Default,
        )
    };
    ($ctx: expr, $colour: ident) => {
        $crate::embed::generate_embed(
            $ctx.author(),
            $ctx.author_member().await,
            $crate::embed::colours::$colour,
        )
    };
    ($ctx: expr, $colour: ident, $guild: expr) => {{
        let mut embed = $crate::embed::generate_embed(
            $ctx.author(),
            $ctx.author_member().await,
            $crate::embed::colours::$colour,
        );

        if $guild {
            if let Some(url) = $ctx.guild().map(|guild| guild.icon_url()).flatten() {
                embed.thumbnail(url);
            }
        }

        embed
    }};
}
