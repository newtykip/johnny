use crate::build_data::FEATURES;
use common::preludes::event::*;
#[cfg(johnny)]
use poise::serenity_prelude::Activity;
use poise::serenity_prelude::Ready;

pub async fn ready(#[cfg(any(johnny, sqlite))] ctx: &Context, ready: &Ready) -> Result<()> {
    // list enabled features
    if !FEATURES.is_empty() {
        logger::info(
            logger::components![
                "Enabled features: " => Bold,
                FEATURES.join(", ") => None
            ],
            None,
        )
        .await?;
    }

    // log that the bot is ready
    logger::info(
        logger::components![
            "Logged in as " => None,
            ready.user.name.clone() => Bold
        ],
        None,
    )
    .await?;

    // set the activity
    #[cfg(johnny)]
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    // ? is 100 really a sane quantity?
    #[cfg(sqlite)]
    if ctx.cache.guild_count() > 100 {
        logger::warn(
            logger::components!["You are in over 100 guilds. Perhaps you should swap from sqlite?"],
            None,
        )
        .await?;
    }

    Ok(())
}
