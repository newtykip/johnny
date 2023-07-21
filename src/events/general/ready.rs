use johnny::{logger::Style, preludes::event::*};
#[cfg(johnny)]
use poise::serenity_prelude::Activity;
use poise::serenity_prelude::Ready;

pub async fn run(
    #[cfg(any(johnny, sqlite))] ctx: &Context,
    ready: &Ready,
    data: &Data,
) -> Result<()> {
    data.logger
        .info(
            vec![
                ("Logged in as ".into(), None),
                (ready.user.name.clone(), Some(Style::default().bold())),
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
        data.logger.warn(vec![("hi".into(), None)], None).await?;
    }

    Ok(())
}
