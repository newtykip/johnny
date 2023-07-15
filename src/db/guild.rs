pub use super::entity::guild::{ActiveModel, Entity, Model};
use super::GetDB;
use async_trait::async_trait;
use poise::serenity_prelude::{Guild, GuildId};
use sea_orm::{ActiveValue::*, DatabaseConnection, DbErr, EntityTrait, InsertResult};

async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(db).await
}

async fn get_db(db: &DatabaseConnection, id: GuildId) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id.to_string()).one(db).await
}

async fn create_db(
    db: &DatabaseConnection,
    id: GuildId,
) -> Result<InsertResult<ActiveModel>, DbErr> {
    let model = ActiveModel {
        id: Set(id.to_string()),
        ..Default::default()
    };

    Entity::insert(model).exec(db).await
}

#[async_trait]
impl GetDB<Model, ActiveModel> for Guild {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        get_db_all(db).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
        get_db(db, self.id).await
    }

    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>, DbErr> {
        create_db(db, self.id).await
    }
}

#[async_trait]
impl GetDB<Model, ActiveModel> for GuildId {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        get_db_all(db).await
    }

    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
        get_db(db, *self).await
    }

    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>, DbErr> {
        create_db(db, *self).await
    }
}
