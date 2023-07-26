use super::{
    create_db,
    entity::member::{ActiveModel, Column, Entity, Model},
    get_db_all, GetDB,
};
use crate::{preludes::general::*, EPOCH};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use rustflake::Snowflake;
use sea_orm::{
    ActiveModelTrait, ActiveValue::*, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait,
    InsertResult, IntoActiveModel, ModelTrait, QueryFilter,
};

static mut SNOWFLAKE_GENERATOR: Lazy<Snowflake> = Lazy::new(|| Snowflake::new(EPOCH, 2, 1));
const ITEM: &str = "member";

#[allow(clippy::needless_update)]
fn default_model(id: String, user_id: String, guild_id: String) -> ActiveModel {
    ActiveModel {
        id: Set(id),
        user_id: Set(user_id),
        guild_id: Set(guild_id),
        ..Default::default()
    }
}

#[async_trait]
impl GetDB<ActiveModel> for Member {
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        let id = unsafe { SNOWFLAKE_GENERATOR.generate() }.to_string();

        create_db(
            db,
            ITEM,
            &id,
            default_model(
                id.clone(),
                self.user.id.to_string(),
                self.guild_id.to_string(),
            ),
        )
        .await
    }

    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        get_db_all::<Entity>(db, ITEM).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
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
            self.create_db(db).await?;
            self.get_db(db).await
        }
    }

    async fn update_db<F>(&self, db: &DatabaseConnection, modify: F) -> Result<Option<Model>>
    where
        F: Send + FnOnce(&mut ActiveModel) -> &mut ActiveModel,
    {
        Ok(if let Some(model) = self.get_db(db).await? {
            Some(
                modify(&mut model.into_active_model())
                    .clone()
                    .update(db)
                    .await
                    .wrap_err({
                        format!(
                            "failed to update member with user id {}, guild id {} in db",
                            self.user.id, self.guild_id
                        )
                    })?,
            )
        } else {
            None
        })
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<Option<DeleteResult>> {
        Ok(if let Some(model) = self.get_db(db).await? {
            Some(model.delete(db).await.wrap_err({
                format!(
                    "failed to delete member with user id {}, guild id {} from db",
                    self.user.id, self.guild_id
                )
            })?)
        } else {
            None
        })
    }
}
