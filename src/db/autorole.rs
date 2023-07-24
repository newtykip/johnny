use super::{entity::autorole::{ActiveModel, Entity}, GetDB};
use crate::{preludes::general::*, EPOCH};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use poise::serenity_prelude::Role;
use rustflake::Snowflake;
use sea_orm::{ActiveValue::*, DatabaseConnection, EntityTrait, InsertResult, ModelTrait};

static mut SNOWFLAKE_GENERATOR: Lazy<Snowflake> = Lazy::new(|| Snowflake::new(EPOCH, 1, 1));

#[async_trait]
pub trait AutoroleDB {
    async fn create_autorole(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>>;
    async fn delete_autorole(&self, db: &DatabaseConnection) -> Result<()>;
}

#[async_trait]
impl AutoroleDB for Role {
    async fn create_autorole(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        self.guild_id.get_db(db).await?;

        let model = ActiveModel {
            id: Set(unsafe { SNOWFLAKE_GENERATOR.generate() }.to_string()),
            guild_id: Set(self.guild_id.to_string()),
            role_id: Set(self.id.to_string()),
            ..Default::default()
        };

        Entity::insert(model).exec(db).await.wrap_err(format!(
            "failed to insert autorole with id {} into db",
            self.id
        ))
    }

    async fn delete_autorole(&self, db: &DatabaseConnection) -> Result<()> {
        if let Some(model) = Entity::find_by_id(self.id.to_string())
            .one(db)
            .await
            .wrap_err(format!(
                "failed to fetch autorole with id {} from db",
                self.id
            ))?
        {
            model.delete(db).await.wrap_err(format!(
                "failed to delete autorole with id {} from db",
                self.id
            ))?;
        }

        Ok(())
    }
}
