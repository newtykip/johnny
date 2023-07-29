use common::preludes::event::*;
use db::sticky::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashSet;

pub async fn add_roles(ctx: &Context, member: &mut Member, db: &DatabaseConnection) -> Result<()> {
    let role_ids = Entity::find()
        .filter(Column::GuildId.eq(member.guild_id.to_string()))
        .filter(Column::UserId.eq(member.user.id.to_string()))
        .all(db)
        .await?
        .par_iter()
        .map(|sticky| RoleId(sticky.role_id.parse().unwrap()))
        .collect::<HashSet<_>>();

    for id in role_ids {
        member.add_role(&ctx.http, id).await?;
    }

    Ok(())
}
