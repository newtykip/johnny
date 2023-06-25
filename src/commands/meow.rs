use crate::{Context, Error};
use johnny::set_embed_author;
use poise::CreateReply;
use rand::seq::SliceRandom;
use serenity::{builder::CreateEmbed, utils::Colour};

fn handle_embed<'a, 'b>(
    msg: &'b mut CreateReply<'a>,
    embed: &CreateEmbed,
) -> &'b mut CreateReply<'a> {
    msg.embeds.push(embed.clone());
    msg
}

/// meow! (checks ping)
#[poise::command(slash_command)]
pub async fn meow(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    println!("{:?}", ctx.data().johnny_images);

    // create the base embed and reply
    let mut embed = CreateEmbed::default();

    set_embed_author(&ctx, &mut embed).await;

    embed
        .title("meow!")
        .colour(Colour::from_rgb(192, 238, 255))
        .image(
            ctx.data()
                .johnny_images
                .choose(&mut rand::thread_rng())
                .unwrap(),
        );

    let reply = ctx.send(|msg| handle_embed(msg, &embed)).await?;

    // work out the ping
    let ping =
        reply.message().await?.timestamp.timestamp_millis() - ctx.created_at().timestamp_millis();

    // edit the reply
    embed.title(format!("meow! ({} ms)", ping));

    reply.edit(ctx, |msg| handle_embed(msg, &embed)).await?;

    Ok(())
}
