// todo: have models in memory which are occasionally synced with the database to reduce writes. drop these on an interval to ensure a low memory overhead.

#[cfg(autorole)]
pub mod autorole;
mod entity;
#[cfg(events)]
pub mod events;
pub mod guild;
mod macros;
pub mod member;
pub mod prelude;
#[cfg(sticky)]
pub mod sticky;
pub mod user;

use async_recursion::async_recursion;
use async_trait::async_trait;
use common::{prelude::*, EPOCH};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, PrimaryKeyTrait};
use std::fmt::Display;
#[cfg(any(autorole, sticky))]
use {once_cell::sync::Lazy, rustflake::Snowflake};

#[cfg(any(autorole, sticky))]
static mut SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| Snowflake::new(EPOCH, 0, 0));

#[cfg(any(autorole, sticky))]
pub fn generate_id() -> String {
    unsafe { SNOWFLAKE.generate() }.to_string()
}

#[async_recursion]
async fn get_db<A, I>(
    db: &DatabaseConnection,
    item: &str,
    id: &I,
    model: A,
) -> Result<Option<<A::Entity as EntityTrait>::Model>>
where
    A: ActiveModelTrait + Clone + Send + Sync,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + Display
        + Clone
        + Sync
        + Send,
{
    if let Some(model) = A::Entity::find_by_id::<I>(id.clone())
        .one(db)
        .await
        .wrap_err(format!("failed to fetch {item} swith id {id} from db"))?
    {
        Ok(Some(model))
    } else {
        A::Entity::insert(model.clone()).exec(db).await?;
        get_db(db, item, id, model).await
    }
}

#[async_trait]
pub trait DB<M: ModelTrait> {
    async fn db(&self, db: &DatabaseConnection) -> Result<Option<M>>;
}
