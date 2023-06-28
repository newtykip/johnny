use crate::{Context, Error};
#[cfg(feature = "johnny")]
use johnny::johnny_image;
use johnny::{apply_embed, create_embed};

async fn run(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    // create the base embed and reply
    let mut embed = create_embed(
        ctx.author(),
        ctx.author_member().await.map(|x| x.into_owned()),
    )
    .await;

    embed.title("meow!");

    // if the johnny feature is enabled, add a random johnny image
    #[cfg(feature = "johnny")]
    embed.image(johnny_image(&ctx.data()));

    let reply = ctx.send(|msg| apply_embed(msg, &embed)).await?;

    // work out the ping
    let ping =
        reply.message().await?.timestamp.timestamp_millis() - ctx.created_at().timestamp_millis();

    // edit the reply
    embed.title(format!("meow! ({} ms)", ping));

    reply.edit(ctx, |msg| apply_embed(msg, &embed)).await?;

    Ok(())
}

/// checks ping
#[cfg(not(feature = "johnny"))]
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    run(ctx).await
}

/// meow! (checks ping)
#[cfg(feature = "johnny")]
#[poise::command(slash_command, rename = "meow")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    run(ctx).await
}
