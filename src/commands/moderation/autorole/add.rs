use johnny::preludes::command::*;

async fn role_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<String> {
    ctx.guild()
        .unwrap()
        .roles
        .iter()
        .map(|(_, role)| role)
        .filter(|role| !role.is_everyone())
        .filter(|role| !role.managed)
        .filter(|role| role.name.starts_with(partial))
        .map(|role| role.name.clone())
        .collect()
}

/// Add an autorole
#[command(
    slash_command,
    category = "moderation",
    guild_only,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_ROLES"
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The role to add"]
    #[autocomplete = "role_autocomplete"]
    role: String,
) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.guild().unwrap();
    let role = guild.role_by_name(&role).unwrap();

    // create the autorole entry
    let entry = role.create_autorole(&ctx.data().db).await;

    if let Err(err) = entry {
        println!("{:?}", err);
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

    set_guild_thumbnail(&mut base_embed, guild.clone());

    base_embed.title("Added autorole!").description(format!(
        "The role {} has been added as an autorole!",
        role.mention()
    ));

    // announce the new autorole
    ctx.send(|msg| msg.embed(|embed| message_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
