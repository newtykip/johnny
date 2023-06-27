use johnny::{logger::Entry, Data};
use poise::serenity_prelude::{Activity, Context, Ready};

pub async fn run(ctx: &Context, _ready: &Ready, data: &Data) {
    // todo: logger
    data.logger
        .send(Entry::info(format!("Logged in as {}", _ready.user.name)))
        .await
        .unwrap();

    // set the activity
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;
}
