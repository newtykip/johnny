use johnny::{Error, Reactions};
use poise::serenity_prelude::{Context, GuildChannel};

pub async fn run(ctx: &Context, thread: &GuildChannel) -> Result<(), Error> {
    // find the post
    let post = &thread.messages(&ctx, |msgs| msgs.limit(1)).await?[0]; // we know there will be a message - the initial suggestion
    let reactions = Reactions::default();

    // upvote reaction
    post.react(&ctx.http, reactions.upvote).await?;

    // downvote reaction
    post.react(&ctx.http, reactions.downvote).await?;

    Ok(())
}
