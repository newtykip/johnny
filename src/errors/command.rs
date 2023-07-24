use color_eyre::Report;
use johnny::{
    embed::{colours, generate_embed, set_guild_thumbnail},
    message_embed,
    preludes::general::*,
    Context,
};

pub async fn run(error: Report, ctx: Context<'_>) {
    // log the error
    logger::error(
        components![
            "Error in command " => Bold,
            ctx.command().qualified_name.clone() => Bold,
            format!(": {}", error) => None
        ],
        Some(&ctx),
    )
    .await
    .unwrap();

    // create the embed
    let mut base_embed = generate_embed(
        ctx.author(),
        ctx.author_member().await,
        Some(colours::FAILURE),
    );

    if let Some(guild) = ctx.guild() {
        set_guild_thumbnail(&mut base_embed, guild);
    }

    base_embed.title("Error!").description(error);

    // announce the error
    ctx.send(|msg| msg.embed(|embed| message_embed!(embed, base_embed)))
        .await
        .unwrap();
}
