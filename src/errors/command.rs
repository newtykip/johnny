use color_eyre::Report;
use johnny::{generate_embed, preludes::general::*, use_embed, Context};

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
    let mut base_embed = generate_embed!(ctx, Failure, true);

    base_embed.title("Error!").description(error);

    // announce the error
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await
        .unwrap();
}
