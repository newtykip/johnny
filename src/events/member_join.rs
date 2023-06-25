use johnny::{create_embed, johnny_image, Data};
use poise::serenity_prelude::Member;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, member: &Member, data: &Data) {
    let mut embed = create_embed(&member.user, Some(member.clone())).await;
    embed.image(johnny_image(&data)).title("Welcome ♡");

    let message = member
        .user
        .direct_message(&ctx.http, |msg| msg.set_embed(embed))
        .await;

    if let Ok(message) = message {
        message.react(&ctx.http, '👋').await.ok();
    } else {
        println!("Failed to send welcome message to {}", member.user.tag());
    }
}
