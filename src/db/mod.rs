// todo: have models in memory which are occasionally synced with the database to reduce writes. drop these on an interval to ensure a low memory overhead.

#[cfg(autorole)]
mod autorole;
mod entity;
mod guild;
mod member;
mod user;

use crate::preludes::eyre::*;
use async_trait::async_trait;
#[cfg(autorole)]
pub use autorole::AutoroleDB;
use sea_orm::{ActiveModelTrait, DatabaseConnection, InsertResult, ModelTrait};

#[async_trait]
pub trait GetDB<M: ModelTrait, A: ActiveModelTrait> {
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<M>>;
    async fn get_db(&self, db: &DatabaseConnection) -> Result<Option<M>>;
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<A>>;
    async fn delete_db(&self, db: &DatabaseConnection) -> Result<()>;
}
