use poise::serenity_prelude::{Activity, Context, Ready};

pub async fn run(ctx: &Context, ready: &Ready) {
    println!("{} is connected!", ready.user.name);

    // set the activity
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;
}
