use crate::Reactions;
use common::event::*;
use poise::serenity_prelude::{Context, GuildChannel};

pub async fn suggestion(ctx: &Context, thread: &GuildChannel) -> Result<()> {
    // find the post
    let post = &thread.messages(&ctx, |msgs| msgs.limit(1)).await?[0];
    let reactions = Reactions::default();

    // upvote reaction
    post.react(&ctx.http, reactions.upvote).await?;

    // downvote reaction
    post.react(&ctx.http, reactions.downvote).await?;

    Ok(())
}
