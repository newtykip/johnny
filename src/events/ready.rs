use johnny::Data;
use poise::serenity_prelude::{Activity, Context, Ready};

pub async fn run(ctx: &Context, _ready: &Ready, data: &Data) {
    data.logger
        .info(format!("Logged in as {}", _ready.user.name))
        .await;

    // set the activity
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;
}
