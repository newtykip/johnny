pub mod events;

use common::preludes::command::*;
use db::{prelude::*, sticky};

/// Toggle sticky roles on and off
#[command(slash_command, category = "moderation", guild_only)]
pub async fn toggle(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let db = &ctx.data().db;
    let guild = ctx.guild().unwrap();

    // enable sticky roles
    let model = {
        let temp = find_one!(guild, db, guild.id)?;
        update!(temp, db, sticky => !temp.sticky)?
    };

    // if sticky roles has been turned off, remove all sticky role records
    if !model.sticky {
        for sticky in sticky::Entity::find()
            .filter(sticky::Column::GuildId.eq(guild.id.to_string()))
            .all(db)
            .await?
        {
            sticky.delete(db).await?;
        }
    }

    // create the embed
    let mut base_embed = if model.sticky {
        generate_embed!(ctx, Success, true)
    } else {
        generate_embed!(ctx, Failure, true)
    };

    base_embed
        .title(if model.sticky {
            "Enabled!"
        } else {
            "Disabled!"
        })
        .description(format!(
            "Sticky roles have been {}!",
            if model.sticky { "enabled" } else { "disabled" }
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
