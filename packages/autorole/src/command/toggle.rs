use common::command::*;
use common::db::prelude::*;

/// Toggle autorole on and off
#[command(
    slash_command,
    category = "moderation",
    guild_only,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn toggle(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let db = &ctx.data().pool;
    let guild = ctx.guild().unwrap();

    // toggle autorole
    let model = select!(Guild, db, Id | guild.id.to_string()).unwrap();
    let autorole = !model.autorole;

    update!(Guild, db, Id | guild.id.to_string(); Autorole => autorole)?;

    // todo: should autoroles be deleted after being disabled?

    // create the embed
    let mut base_embed = if autorole {
        generate_embed!(ctx, Success, true)
    } else {
        generate_embed!(ctx, Failure, true)
    };

    base_embed
        .title(if autorole { "Enabled!" } else { "Disabled!" })
        .description(format!(
            "Autorole has been {}!",
            if autorole { "enabled" } else { "disabled" }
        ));

    // announce the toggle
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
