#[cfg(johnny)]
use johnny::johnny_image;
use johnny::preludes::command::*;

async fn run(ctx: Context<'_>) -> Result<()> {
    #[cfg(not(johnny))]
    ctx.defer_ephemeral().await?;

    let base_embed = generate_embed(ctx.author(), ctx.author_member().await, None);

    // if the johnny feature is enabled, add a random johnny image
    #[cfg(johnny)]
    let (number, johnny_image) = johnny_image(&ctx.data());
    #[cfg(johnny)]
    let footer_text = format!(
        "Image number {} out of {}",
        number,
        ctx.data().johnny_images.len()
    );

    let reply = ctx
        .send(|msg| {
            msg.embed(|embed| {
                embed.clone_from(&base_embed);

                cfg_if! {
                    if #[cfg(johnny)] {
                        embed
                            .title("meow!")
                            .image(&johnny_image)
                            .footer(|footer| footer.text(&footer_text))
                    } else {
                        embed.title("ping!")
                    }
                }
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

                cfg_if! {
                    if #[cfg(johnny)] {
                        embed
                            .title(format!("meow! ({} ms)", ping))
                            .image(&johnny_image)
                            .footer(|footer| footer.text(&footer_text))
                    } else {
                        embed.title(format!("ping! ({} ms)", ping))
                    }
                }
            })
        })
        .await?;

    Ok(())
}

cfg_if! {
    if #[cfg(johnny)] {
        /// meow! (checks ping)
        #[command(slash_command, rename = "meow")]
        pub async fn ping(ctx: Context<'_>) -> Result<()> {
            run(ctx).await
        }
    } else {
        /// checks ping
        #[command(slash_command)]
        pub async fn ping(ctx: Context<'_>) -> Result<()> {
            run(ctx).await
        }
    }
}
