use super::entity::guild::{ActiveModel, Entity, Model};
use super::GetDB;
use crate::preludes::eyre::*;
use async_trait::async_trait;
use poise::serenity_prelude::{Guild, GuildId};
use sea_orm::{ActiveValue::*, DatabaseConnection, EntityTrait, InsertResult, ModelTrait};

async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
    Entity::find()
        .all(db)
        .await
        .wrap_err("failed to fetch all guilds from db")
}

async fn get_db(db: &DatabaseConnection, id: &GuildId) -> Result<Option<Model>> {
    Entity::find_by_id(id.to_string())
        .one(db)
        .await
        .wrap_err(format!("failed to fetch guild with id {} from db", id))
}

async fn create_db(db: &DatabaseConnection, id: &GuildId) -> Result<InsertResult<ActiveModel>> {
    let model = ActiveModel {
        id: Set(id.to_string()),
        ..Default::default()
    };

    Entity::insert(model)
        .exec(db)
        .await
        .wrap_err(format!("failed to insert guild with id {} into db", id))
}

async fn delete_db(db: &DatabaseConnection, id: &GuildId) -> Result<()> {
    if let Some(model) = get_db(db, id).await? {
        model
            .delete(db)
            .await
            .wrap_err(format!("failed to delete guild with id {} from db", id))?;
    }

    Ok(())
}

#[async_trait]
impl GetDB<Model, ActiveModel> for Guild {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        get_db_all(db).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db(db, &self.id).await
    }

    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        create_db(db, &self.id).await
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<()> {
        delete_db(db, &self.id).await
    }
}

#[async_trait]
impl GetDB<Model, ActiveModel> for GuildId {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
        get_db_all(db).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db(db, self).await
    }

    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>> {
        create_db(db, self).await
    }

    async fn delete_db(&self, db: &DatabaseConnection) -> Result<()> {
        delete_db(db, self).await
    }
}
