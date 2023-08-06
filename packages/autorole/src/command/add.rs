use common::command::*;
use common::db::prelude::*;

async fn role_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<AutocompleteChoice<String>> {
    ctx.guild()
        .unwrap()
        .roles
        .values()
        .filter(|role| !role.is_everyone())
        .filter(|role| !role.managed)
        .filter(|role| role.name.starts_with(partial))
        .map(|role| AutocompleteChoice {
            name: role.name.clone(),
            value: role.id.to_string(),
        })
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
    #[rename = "role"]
    role_id: String,
) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let guild = ctx.guild().unwrap();
    let role = guild.roles.get(&RoleId(role_id.parse()?)).unwrap();
    insert!(Autorole, &ctx.data().pool, Id => generate_id(), GuildId => guild.id.to_string(), RoleId => role.id.to_string())?;

    // create the embed
    let mut base_embed = generate_embed!(ctx, Success, true);

    base_embed.title("Added autorole!").description(format!(
        "The role {} has been added as an autorole!",
        role.mention()
    ));

    // announce the new autorole
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
