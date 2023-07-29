use crate::{prelude::*, user::*};
use common::preludes::event::*;

pub async fn create_user(user: &User, db: &DatabaseConnection) -> Result<()> {
    let id = user.id.to_string();

    if let None = Entity::find_by_id(&id).one(db).await? {
        let model = ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        model.insert(db).await?;
    }

    Ok(())
}
