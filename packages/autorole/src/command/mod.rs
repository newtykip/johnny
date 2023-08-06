use common::{command::*, load_command};

load_command!(toggle, add, remove);

/// Modify current autorole settings
#[command(
    slash_command,
    subcommands("toggle", "add", "remove"),
    category = "moderation",
    guild_only
)]
pub async fn autorole(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
