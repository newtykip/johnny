use johnny::{Data, Error, DOWNVOTE_REACTION, UPVOTE_REACTION};
use poise::serenity_prelude::{Context, GuildChannel};

pub async fn run(ctx: &Context, thread: &GuildChannel, _data: &Data) -> Result<(), Error> {
    // find the post
    let post = &thread.messages(&ctx, |msgs| msgs.limit(1)).await?[0]; // we know there will be a message - the initial suggestion

    // upvote reaction
    post.react(&ctx.http, UPVOTE_REACTION.clone()).await?;

    // downvote reaction
    post.react(&ctx.http, DOWNVOTE_REACTION.clone()).await?;

    Ok(())
}
