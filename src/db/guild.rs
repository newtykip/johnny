use super::{
    create_db, delete_db,
    entity::autorole,
    entity::guild::{ActiveModel, Entity, Model},
    get_db, get_db_all, update_db, GetDB,
};
use crate::preludes::general::*;
use async_trait::async_trait;
use sea_orm::{
    ActiveValue::*, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, InsertResult,
    QueryFilter,
};

const ITEM: &str = "guild";

#[async_trait]
pub trait GetAutoroles {
    async fn get_autorole(
        &self,
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<autorole::Model>>;
    async fn get_all_autoroles(&self, db: &DatabaseConnection) -> Result<Vec<autorole::Model>>;
}

async fn get_autorole(db: &DatabaseConnection, id: String) -> Result<Option<autorole::Model>> {
    autorole::Entity::find_by_id(&id)
        .one(db)
        .await
        .wrap_err(format!("failed to fetch autorole with id {}from db", id))
}

async fn get_all_autoroles(
    db: &DatabaseConnection,
    guild_id: String,
) -> Result<Vec<autorole::Model>> {
    autorole::Entity::find()
        .filter(autorole::Column::GuildId.eq(guild_id.clone()))
        .all(db)
        .await
        .wrap_err(format!(
            "failed to fetch autoroles from guild with id {} from db",
            guild_id
        ))
}

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
impl GetAutoroles for Guild {
    async fn get_autorole(
        &self,
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<autorole::Model>> {
        get_autorole(db, id).await
    }

    async fn get_all_autoroles(&self, db: &DatabaseConnection) -> Result<Vec<autorole::Model>> {
        get_all_autoroles(db, self.id.to_string()).await
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

#[async_trait]
impl GetAutoroles for GuildId {
    async fn get_autorole(
        &self,
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<autorole::Model>> {
        get_autorole(db, id).await
    }

    async fn get_all_autoroles(&self, db: &DatabaseConnection) -> Result<Vec<autorole::Model>> {
        get_all_autoroles(db, self.to_string()).await
    }
}
