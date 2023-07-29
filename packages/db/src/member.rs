use crate::prelude::*;
pub use crate::{entity::member::*, generate_id};
use async_trait::async_trait;
use common::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[async_trait]
impl DB<Model> for Member {
    async fn db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        let user_id = self.user.id;
        let guild_id = self.guild_id;

        if let Some(model) = Entity::find()
            .filter(Column::UserId.eq(user_id.to_string()))
            .filter(Column::GuildId.eq(guild_id.to_string()))
            .one(db)
            .await
            .wrap_err({
                format!(
                    "failed to fetch member with user id {}, guild id {} from db",
                    user_id, guild_id
                )
            })?
        {
            Ok(Some(model))
        } else {
            Entity::insert(ActiveModel {
                id: Set(generate_id()),
                user_id: Set(user_id.to_string()),
                guild_id: Set(guild_id.to_string()),
                ..Default::default()
            })
            .exec(db)
            .await?;

            self.db(db).await
        }
    }
}
