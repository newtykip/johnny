use crate::{guild::*, prelude::*};
use common::preludes::event::*;

pub async fn remove_guild(guild: &UnavailableGuild, db: &DatabaseConnection) -> Result<()> {
    let model_opt = Entity::find_by_id(guild.id.to_string()).one(db).await?;

    if let Some(model) = model_opt {
        model.delete(db).await?;
    }

    Ok(())
}
