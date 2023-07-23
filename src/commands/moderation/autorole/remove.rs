use johnny::preludes::command::*;

// todo: implement

/// Remove an autorole
#[command(slash_command, category = "moderation")]
pub async fn remove(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
