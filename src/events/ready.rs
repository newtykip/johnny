use johnny::{Data, Error};
use poise::serenity_prelude::Ready;
#[cfg(feature = "johnny")]
use poise::serenity_prelude::{Activity, Context};

pub async fn run(
    #[cfg(feature = "johnny")] ctx: &Context,
    ready: &Ready,
    data: &Data,
) -> Result<(), Error> {
    data.logger
        .info(format!("Logged in as {}", ready.user.name), None)
        .await;

    #[cfg(feature = "johnny")]
    // set the activity
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    Ok(())
}
