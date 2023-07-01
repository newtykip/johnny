use johnny::Error;
use poise::serenity_prelude::Message;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, message: &Message) -> Result<(), Error> {
    // get all messages before the new message
    let messages = message
        .channel(&ctx)
        .await?
        .guild()
        .expect("the usernames channel is in a guild")
        .messages(&ctx, |messages| messages.before(message.id))
        .await?;

    // has the user already posted?
    let has_posted = messages
        .iter()
        .any(|msg| msg.author.id == message.author.id);

    // if they have, delete the message
    if has_posted {
        message.delete(&ctx).await?;
    }

    Ok(())
}
