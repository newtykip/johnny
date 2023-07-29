use crate::{member, prelude::*, user};
use common::preludes::event::*;

pub async fn remove_user(user: &User, db: &DatabaseConnection) -> Result<()> {
    let id = user.id.to_string();
    let member_models = member::Entity::find()
        .filter(member::Column::UserId.eq(&id))
        .all(db)
        .await?;

    if member_models.len() == 0 {
        let model_opt = user::Entity::find_by_id(id).one(db).await?;

        if let Some(model) = model_opt {
            model.delete(db).await?;
        }
    }

    Ok(())
}
