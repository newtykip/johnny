use johnny::preludes::event::*;
use owo_colors::OwoColorize;
#[cfg(johnny)]
use poise::serenity_prelude::Activity;
use poise::serenity_prelude::Ready;

pub async fn run(
    #[cfg(any(johnny, sqlite))] ctx: &Context,
    ready: &Ready,
    data: &Data,
) -> Result<()> {
    data.logger
        .info(format!("Logged in as {}", ready.user.name.bold()), None)
        .await?;

    // set the activity
    #[cfg(johnny)]
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    // ? is 100 really a sane quantity?
    #[cfg(sqlite)]
    if ctx.cache.guild_count() > 100 {
        data.logger
            .warn(
                format!(
                    "Your server is in {} guilds. Consider migrating from sqlite to either postgres or mysql.",
                    ctx.cache.guild_count()
                ),
                None,
            )
            .await?;
    }

    Ok(())
}
