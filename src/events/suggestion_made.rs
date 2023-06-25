use johnny::{DOWNVOTE_ID, UPVOTE_ID};
use poise::serenity_prelude::{Context, GuildChannel};

pub async fn run(ctx: &Context, thread: &GuildChannel) {
    // find the post
    let post = thread
        .message(&ctx.http, thread.last_message_id.unwrap())
        .await
        .unwrap(); // we know there will be a message - the initial suggestion

    // upvote reaction
    post.react(&ctx.http, UPVOTE_ID).await.unwrap();

    // downvote reaction
    post.react(&ctx.http, DOWNVOTE_ID).await.unwrap();
}
