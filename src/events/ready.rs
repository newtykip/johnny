use johnny::{Data, Error};
#[cfg(feature = "johnny")]
use poise::serenity_prelude::Activity;
#[cfg(any(feature = "johnny", feature = "sqlite"))]
use poise::serenity_prelude::Context;
use poise::serenity_prelude::Ready;

pub async fn run(
    #[cfg(any(feature = "johnny", feature = "sqlite"))] ctx: &Context,
    ready: &Ready,
    data: &Data,
) -> Result<(), Error> {
    data.logger
        .info(format!("Logged in as {}", ready.user.name), None)
        .await;

    // set the activity
    #[cfg(feature = "johnny")]
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    // ? is 100 really a sane quantity?
    #[cfg(feature = "sqlite")]
    if ctx.cache.guild_count() > 100 {
        data.logger
            .warn(
                format!(
                    "Your server is in {} guilds. Consider migrating from sqlite to either postgres or mysql.",
                    ctx.cache.guild_count()
                ),
                None,
            )
            .await;
    }

    Ok(())
}
