use std::collections::HashSet;

use common::preludes::event::*;
use db::{autorole::*, prelude::*};
use sea_orm::DatabaseConnection;

pub async fn apply_role(ctx: &Context, member: &mut Member, db: &DatabaseConnection) -> Result<()> {
    if let Some(guild) = member.guild_id.db(db).await? {
        if guild.autorole {
            let roles = Entity::find()
                .all(db)
                .await?
                .iter()
                .map(|x| RoleId(x.role_id.parse().unwrap()))
                .collect::<HashSet<_>>();

            for id in roles {
                member.add_role(&ctx.http, id).await?;
            }
        }
    }

    Ok(())
}
