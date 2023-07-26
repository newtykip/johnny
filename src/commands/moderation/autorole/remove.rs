use johnny::preludes::command::*;
use sea_orm::ModelTrait;

async fn autorole_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<AutocompleteChoice<String>> {
    let guild = ctx.guild().unwrap();

    guild
        .get_all_autoroles(&ctx.data().db)
        .await
        .unwrap()
        .iter()
        .map(|autorole| {
            let id = autorole.id.clone();
            let name = guild
                .roles
                .get(&RoleId(autorole.role_id.parse().unwrap()))
                .unwrap()
                .name
                .clone();

            (name, id)
        })
        .filter(|(name, _)| name.starts_with(partial))
        .map(|(name, value)| AutocompleteChoice { name, value })
        .collect::<Vec<_>>()
}

/// Remove an autorole
#[command(slash_command, category = "moderation")]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The autorole to remove"]
    #[autocomplete = "autorole_autocomplete"]
    #[rename = "role"]
    autorole_id: String,
) -> Result<()> {
    ctx.defer_ephemeral().await?;

    // get the autorole
    let db = &ctx.data().db;
    let guild = ctx.guild().unwrap();
    let autorole = guild.get_autorole(db, autorole_id).await?.unwrap();

    // delete it
    autorole.clone().delete(db).await?;

    // create the embed
    let mut base_embed = generate_embed!(ctx, Success, true);

    base_embed.title("Removed autorole!").description(format!(
        "The role {} has been removed as an autorole!",
        guild
            .roles
            .get(&RoleId(autorole.role_id.parse()?))
            .unwrap()
            .mention()
    ));

    // announce the removal of the autorole
    ctx.send(|msg| msg.embed(|embed| use_embed!(embed, base_embed)))
        .await?;

    Ok(())
}
