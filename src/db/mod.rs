// todo: have models in memory which are occasionally synced with the database to reduce writes. drop these on an interval to ensure a low memory overhead.

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, InsertResult, ModelTrait};

mod entity;
pub mod guild;
pub mod user;

#[async_trait]
pub trait GetDB<M: ModelTrait, A: ActiveModelTrait> {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<M>, DbErr>;
    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<M>, DbErr>;
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<A>, DbErr>;
}
