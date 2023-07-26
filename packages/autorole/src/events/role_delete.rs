use common::preludes::event::*;
use db::entity::autorole;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

pub async fn role_delete(role_id: &RoleId, db: &DatabaseConnection) -> Result<()> {
    // delete the associated autorole document
    let model = autorole::Entity::find()
        .filter(autorole::Column::RoleId.eq(role_id.to_string()))
        .one(db)
        .await?;

    if let Some(model) = model {
        model.delete(db).await?;
    }

    Ok(())
}
