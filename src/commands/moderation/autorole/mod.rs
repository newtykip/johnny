use johnny::{load_command, preludes::command::*};

load_command!(toggle add remove list);

/// View or modify current autorole settings
#[command(
    slash_command,
    subcommands("toggle", "add", "remove", "list"),
    category = "moderation",
    guild_only
)]
pub async fn autorole(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
