pub mod events;

use common::command::*;
use common::db::prelude::*;

/// Toggle sticky roles on and off
#[command(slash_command, category = "moderation", guild_only)]
pub async fn toggle(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let pool = &ctx.data().pool;
    let guild = ctx.guild().unwrap();

    // toggle sticky roles
    let model = select!(Guild, pool, Id | guild.id.to_string()).unwrap();
    let sticky = !model.sticky;

    update!(Guild, pool, Id | guild.id.to_string(); Sticky => sticky)?;

    // if sticky roles has been turned off, remove all sticky role records
    if sticky {
        delete!(Sticky, pool, GuildId | guild.id.to_string())?;
    }

    // create the embed
    let mut base_embed = if sticky {
        generate_embed!(ctx, Success, true)
    } else {
        generate_embed!(ctx, Failure, true)
    };

    base_embed
        .title(if sticky { "Enabled!" } else { "Disabled!" })
        .description(format!(
            "Sticky roles have been {}!",
            if sticky { "enabled" } else { "disabled" }
        ));

    // announce the toggle
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await?;

    Ok(())
}

/// Sticky roles
#[command(slash_command, subcommands("toggle"), category = "moderation")]
pub async fn sticky(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
