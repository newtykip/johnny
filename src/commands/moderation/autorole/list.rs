use johnny::preludes::command::*;

// todo: implement

/// List all autorole entries for the current guild
#[command(slash_command, category = "moderation", guild_only)]
pub async fn list(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
