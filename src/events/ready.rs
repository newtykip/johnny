use johnny::{Data, Error};
use poise::serenity_prelude::{Activity, Context, Ready};

pub async fn run(ctx: &Context, _ready: &Ready, data: &Data) -> Result<(), Error> {
    data.logger
        .info(format!("Logged in as {}", _ready.user.name), None)
        .await;

    #[cfg(feature = "johnny")]
    // set the activity
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    Ok(())
}
