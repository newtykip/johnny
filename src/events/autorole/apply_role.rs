use johnny::{
    db::{GetAutoroles, GetDB},
    preludes::event::*,
};
use sea_orm::DatabaseConnection;

pub async fn apply_role(ctx: &Context, member: &mut Member, db: &DatabaseConnection) -> Result<()> {
    if let Some(guild) = member.guild_id.get_db(db).await? {
        if guild.autorole {
            let roles = member
                .guild_id
                .get_all_autoroles(db)
                .await?
                .iter()
                .map(|x| RoleId(x.role_id.parse().unwrap()))
                .collect::<Vec<_>>();

            for id in roles {
                member.add_role(&ctx.http, id).await?;
            }
        }
    }

    Ok(())
}
