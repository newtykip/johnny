use super::{
    create_db, delete_db,
    entity::guild::{ActiveModel, Entity, Model},
    get_db, get_db_all, update_db, GetDB,
};
use crate::preludes::general::*;
use async_trait::async_trait;
use poise::serenity_prelude::{Guild, GuildId};
use sea_orm::{ActiveValue::*, DatabaseConnection, DeleteResult, InsertResult};

const ITEM: &str = "guild";

fn default_model(id: String) -> ActiveModel {
    ActiveModel {
        id: Set(id),
        ..Default::default()
    }
}

#[async_trait]
impl GetDB<ActiveModel> for Guild {
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        create_db(
            db,
            ITEM,
            &self.id.to_string(),
            default_model(self.id.to_string()),
        )
        .await
    }

    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        get_db_all::<Entity>(db, ITEM).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db::<ActiveModel, String>(
            db,
            ITEM,
            &self.id.to_string(),
            Some(default_model(self.id.to_string())),
        )
        .await
    }

    async fn update_db<F>(&self, db: &DatabaseConnection, modify: F) -> Result<Option<Model>>
    where
        F: Send + FnOnce(&mut ActiveModel) -> &mut ActiveModel,
    {
        update_db::<ActiveModel, String, F>(
            db,
            ITEM,
            &self.id.to_string(),
            default_model(self.id.to_string()),
            modify,
        )
        .await
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<Option<DeleteResult>> {
        delete_db::<ActiveModel, String>(db, ITEM, &self.id.to_string()).await
    }
}

#[async_trait]
impl GetDB<ActiveModel> for GuildId {
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        create_db(db, ITEM, &self.to_string(), default_model(self.to_string())).await
    }

    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        get_db_all::<Entity>(db, ITEM).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db::<ActiveModel, String>(
            db,
            ITEM,
            &self.to_string(),
            Some(default_model(self.to_string())),
        )
        .await
    }

    async fn update_db<F>(&self, db: &DatabaseConnection, modify: F) -> Result<Option<Model>>
    where
        F: Send + FnOnce(&mut ActiveModel) -> &mut ActiveModel,
    {
        update_db::<ActiveModel, String, F>(
            db,
            ITEM,
            &self.to_string(),
            default_model(self.to_string()),
            modify,
        )
        .await
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<Option<DeleteResult>> {
        delete_db::<ActiveModel, String>(db, ITEM, &self.to_string()).await
    }
}
