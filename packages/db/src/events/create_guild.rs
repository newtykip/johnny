use crate::{guild::*, prelude::*};
use common::preludes::event::*;

pub async fn create_guild(guild: &Guild, db: &DatabaseConnection) -> Result<()> {
    let model = ActiveModel {
        id: Set(guild.id.to_string()),
        ..Default::default()
    };

    model.insert(db).await?;

    Ok(())
}
