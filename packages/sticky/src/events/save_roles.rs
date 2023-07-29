use common::preludes::event::*;
use db::{generate_id, guild, prelude::*, sticky};

// todo: only on sticky
// todo: fix
pub async fn save_roles(member: &Member, db: &DatabaseConnection) -> Result<()> {
    let sticky = guild::Entity::find_by_id(member.guild_id.to_string())
        .one(db)
        .await?
        .map(|g| g.sticky)
        .unwrap_or(false);

    if sticky {
        let models = member
            .roles
            .par_iter()
            .map(|role_id| sticky::ActiveModel {
                id: Set(generate_id()),
                guild_id: Set(member.guild_id.to_string()),
                user_id: Set(member.user.id.to_string()),
                role_id: Set(role_id.to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        sticky::Entity::insert_many(models).exec(db).await?;
    }

    Ok(())
}
