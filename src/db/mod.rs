// todo: have models in memory which are occasionally synced with the database to reduce writes. drop these on an interval to ensure a low memory overhead.

#[cfg(autorole)]
mod autorole;
pub mod entity;
mod guild;
mod member;
mod user;

use crate::preludes::general::*;
use async_trait::async_trait;
#[cfg(autorole)]
pub use autorole::AutoroleDB;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait,
    InsertResult, IntoActiveModel, PrimaryKeyTrait,
};
use std::fmt::Display;

// c
async fn create_db<A, I>(
    db: &DatabaseConnection,
    item: &str,
    id: &I,
    model: A,
) -> Result<InsertResult<A>>
where
    A: ActiveModelTrait,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Display,
{
    A::Entity::insert(model)
        .exec(db)
        .await
        .wrap_err(format!("failed to insert {item} with id {id} into db"))
}

// r
async fn get_db<E, I>(db: &DatabaseConnection, item: &str, id: &I) -> Result<Option<E::Model>>
where
    E: EntityTrait,
    I: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Display + Clone,
{
    E::find_by_id::<I>(id.clone())
        .one(db)
        .await
        .wrap_err(format!("failed to fetch {item} swith id {id} from db"))
}

async fn get_db_all<E>(db: &DatabaseConnection, item: &str) -> Result<Vec<E::Model>>
where
    E: EntityTrait,
{
    E::find()
        .all(db)
        .await
        .wrap_err(format!("failed to fetch all {item}s from db"))
}

// u
async fn update_db<A, I, F>(
    db: &DatabaseConnection,
    item: &str,
    id: &I,
    modify: F,
) -> Result<Option<<A::Entity as EntityTrait>::Model>>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + Display
        + Clone,
    F: Send + FnOnce(&mut A) -> &mut A,
    <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A>,
{
    Ok(
        if let Some(model) = get_db::<A::Entity, I>(db, item, id).await? {
            Some(
                modify(&mut model.into_active_model())
                    .clone()
                    .update(db)
                    .await
                    .wrap_err(format!("failed to update {item} with id {id} in db"))?,
            )
        } else {
            None
        },
    )
}

// d
async fn delete_db<A, I>(
    db: &DatabaseConnection,
    item: &str,
    id: &I,
) -> Result<Option<DeleteResult>>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + Display
        + Clone,
    <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A>,
{
    Ok(
        if let Some(model) = get_db::<A::Entity, I>(db, "guild", id).await? {
            Some(
                model
                    .into_active_model()
                    .delete(db)
                    .await
                    .wrap_err(format!("failed to delete {item} with id {id} from db"))?,
            )
        } else {
            None
        },
    )
}

#[async_trait]
pub trait GetDB<A: ActiveModelTrait> {
    // c
    async fn create_db(&self, db: &DatabaseConnection) -> Result<InsertResult<A>>;

    // r
    async fn get_db_all(db: &DatabaseConnection) -> Result<Vec<<A::Entity as EntityTrait>::Model>>;
    async fn get_db(
        &self,
        db: &DatabaseConnection,
    ) -> Result<Option<<A::Entity as EntityTrait>::Model>>;

    // u
    async fn update_db<F>(
        &self,
        db: &DatabaseConnection,
        modify: F,
    ) -> Result<Option<<A::Entity as EntityTrait>::Model>>
    where
        F: Send + FnOnce(&mut A) -> &mut A;

    // d
    async fn delete_db(&self, db: &DatabaseConnection) -> Result<Option<DeleteResult>>;
}
