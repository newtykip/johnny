use super::entity::member::{ActiveModel, Column, Entity, Model};
use super::GetDB;
use crate::{preludes::eyre::*, EPOCH};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Member;
use rustflake::Snowflake;
use sea_orm::{
    ActiveValue::*, ColumnTrait, DatabaseConnection, EntityTrait, InsertResult, ModelTrait,
    QueryFilter,
};

static mut SNOWFLAKE_GENERATOR: Lazy<Snowflake> = Lazy::new(|| Snowflake::new(EPOCH, 2, 1));

#[async_trait]
impl GetDB<Model, ActiveModel> for Member {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        Entity::find()
            .all(db)
            .await
            .wrap_err("failed to fetch all users from db")
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        let user_id = self.user.id;
        let guild_id = self.guild_id;

        Entity::find()
            .filter(Column::UserId.eq(user_id.to_string()))
            .filter(Column::GuildId.eq(guild_id.to_string()))
            .one(db)
            .await
            .wrap_err({
                format!(
                    "failed to fetch member with user id {}, guild id {} from db",
                    user_id, guild_id
                )
            })
    }

    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        let user_id = self.user.id;
        let guild_id = self.guild_id;

        let model = ActiveModel {
            id: Set(unsafe { SNOWFLAKE_GENERATOR.generate() }.to_string()),
            user_id: Set(user_id.to_string()),
            guild_id: Set(guild_id.to_string()),
            ..Default::default()
        };

        Entity::insert(model).exec(db).await.wrap_err({
            format!(
                "failed to insert member with user id {}, guild id {} into db",
                user_id, guild_id
            )
        })
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<()> {
        if let Some(model) = self.get_db(db).await? {
            model.delete(db).await.wrap_err({
                format!(
                    "failed to delete member with user id {}, guild id {} from db",
                    self.user.id, self.guild_id
                )
            })?;
        }

        Ok(())
    }
}
