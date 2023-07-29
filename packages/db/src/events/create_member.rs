use crate::{member::*, prelude::*};
use common::preludes::event::*;

pub async fn create_member(member: &Member, db: &DatabaseConnection) -> Result<()> {
    let model = ActiveModel {
        id: Set(generate_id()),
        user_id: Set(member.user.id.to_string()),
        guild_id: Set(member.guild_id.to_string()),
        ..Default::default()
    };

    model.insert(db).await?;

    Ok(())
}
