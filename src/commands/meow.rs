use crate::{Context, Error};
use johnny::{apply_embed, create_embed, johnny_image};

/// meow! (checks ping)
#[poise::command(slash_command)]
pub async fn meow(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    // create the base embed and reply
    let mut embed = create_embed(
        &ctx.author(),
        ctx.author_member().await.map(|x| x.into_owned()),
    )
    .await;

    embed.title("meow!").image(johnny_image(&ctx.data()));

    let reply = ctx.send(|msg| apply_embed(msg, &embed)).await?;

    // work out the ping
    let ping =
        reply.message().await?.timestamp.timestamp_millis() - ctx.created_at().timestamp_millis();

    // edit the reply
    embed.title(format!("meow! ({} ms)", ping));

    reply.edit(ctx, |msg| apply_embed(msg, &embed)).await?;

    Ok(())
}
