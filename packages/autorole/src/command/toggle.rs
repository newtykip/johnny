use common::preludes::command::*;
use db::prelude::*;

/// Toggle autorole on and off
#[command(
    slash_command,
    category = "moderation",
    guild_only,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn toggle(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let db = &ctx.data().db;
    let guild = ctx.guild().unwrap();
    let model = {
        let temp = find_one!(guild, db, guild.id)?;
        update!(temp, db, autorole => !temp.autorole)?
    };

    // create the embed
    let mut base_embed = if model.autorole {
        generate_embed!(ctx, Success, true)
    } else {
        generate_embed!(ctx, Failure, true)
    };

    base_embed
        .title(if model.autorole {
            "Enabled!"
        } else {
            "Disabled!"
        })
        .description(format!(
            "Autorole has been {}!",
            if model.autorole {
                "enabled"
            } else {
                "disabled"
            }
        ));

    // announce the toggle
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
