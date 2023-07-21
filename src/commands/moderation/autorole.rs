use johnny::preludes::command::*;

/// View or modify current autorole settings
#[command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_ROLES",
    guild_only,
    category = "moderation"
)]
pub async fn autorole(ctx: Context<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    // todo: do your job newtykins
    ctx.send(|msg| msg.content("Needs to be implented..."))
        .await?;

    Ok(())
}
