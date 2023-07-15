#[cfg(johnny)]
use johnny::johnny_image;
use johnny::preludes::command::*;

async fn run(ctx: Context<'_>) -> Result<(), Error> {
    #[cfg(not(johnny))]
    ctx.defer_ephemeral().await?;

    let base_embed = generate_base_embed(ctx.author(), ctx.author_member().await);

    // if the johnny feature is enabled, add a random johnny image
    #[cfg(johnny)]
    let johnny_image = johnny_image(&ctx.data());

    let reply = ctx
        .send(|msg| {
            msg.embed(|embed| {
                embed.clone_from(&base_embed);

                #[cfg(johnny)]
                return embed.title("meow!").image(&johnny_image);

                #[cfg(not(johnny))]
                embed.title("ping!")
            })
        })
        .await?;

    // work out the ping
    let ping =
        reply.message().await?.timestamp.timestamp_millis() - ctx.created_at().timestamp_millis();

    // edit the reply
    reply
        .edit(ctx, |msg| {
            msg.embed(|embed| {
                embed.clone_from(&base_embed);

                #[cfg(johnny)]
                return embed
                    .title(format!("meow! ({} ms)", ping))
                    .image(johnny_image);

                #[cfg(not(johnny))]
                embed.title(format!("ping! ({} ms)", ping))
            })
        })
        .await?;

    Ok(())
}

/// checks ping
#[cfg(not(johnny))]
#[command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    run(ctx).await
}

/// meow! (checks ping)
#[cfg(johnny)]
#[command(slash_command, rename = "meow")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    run(ctx).await
}
