use johnny::preludes::command::*;

/// Add an autorole
#[command(
    slash_command,
    category = "moderation",
    guild_only,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_ROLES"
)]
pub async fn add(ctx: Context<'_>, role: Role) -> Result<()> {
    ctx.defer_ephemeral().await?;

    if role.is_everyone() {
        return Err(eyre!("You can't add @everyone as an autorole!"));
    }

    // create the autorole entry
    let entry = role.create_autorole(&ctx.data().db).await;

    if entry.is_err() {
        return Err(eyre!(
            "Failed to create autorole entry, are you sure it doesn't already exist?"
        ));
    }

    entry?;

    // create the embed
    let mut base_embed = generate_embed(
        ctx.author(),
        ctx.author_member().await,
        Some(colours::SUCCESS),
    );

    base_embed.title("Added autorole!").description(format!(
        "The role {} has been added as an autorole!",
        role.mention()
    ));

    // announce the new autorole
    ctx.send(|msg| msg.embed(|embed| message_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
