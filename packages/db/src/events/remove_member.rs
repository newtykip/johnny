use crate::{member::*, prelude::*};
use common::preludes::event::*;

pub async fn remove_member(member: &Member, db: &DatabaseConnection) -> Result<()> {
    let model_opt = Entity::find()
        .filter(Column::GuildId.eq(member.guild_id.to_string()))
        .filter(Column::UserId.eq(member.user.id.to_string()))
        .one(db)
        .await?;

    if let Some(model) = model_opt {
        model.delete(db).await?;
    }

    Ok(())
}
