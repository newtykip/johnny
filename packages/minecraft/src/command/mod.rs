use common::{command::*, load_command};

load_command!(pixel);

#[command(slash_command, subcommands("pixel"), category = "minecraft")]
pub async fn minecraft(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
