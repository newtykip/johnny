use johnny::{Data, Error, DOWNVOTE_ID, UPVOTE_ID};
use poise::serenity_prelude::{Context, GuildChannel};

pub async fn run(ctx: &Context, thread: &GuildChannel, _data: &Data) -> Result<(), Error> {
    // find the post
    let post = thread
        .message(
            &ctx.http,
            thread
                .last_message_id
                .expect("there should be a message in every thread"),
        )
        .await?; // we know there will be a message - the initial suggestion

    // upvote reaction
    post.react(&ctx.http, UPVOTE_ID).await?;

    // downvote reaction
    post.react(&ctx.http, DOWNVOTE_ID).await?;

    Ok(())
}
