// todo: have models in memory which are occasionally synced with the database to reduce writes. drop these on an interval to ensure a low memory overhead.

#[cfg(autorole)]
mod autorole;
pub mod entity;
mod guild;
mod member;
mod user;

use crate::preludes::general::*;
use async_recursion::async_recursion;
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
#[async_recursion]
async fn get_db<A, I>(
    db: &DatabaseConnection,
    item: &str,
    id: &I,
    model: Option<A>,
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
        if let Some(model) = model {
            create_db(db, item, id, model.clone()).await?;
            get_db(db, item, id, Some(model)).await
        } else {
            Ok(None)
        }
    }
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
    model: A,
    modify: F,
) -> Result<Option<<A::Entity as EntityTrait>::Model>>
where
    A: ActiveModelTrait + ActiveModelBehavior + Send + Sync,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + Display
        + Clone
        + Sync
        + Send,
    F: Send + FnOnce(&mut A) -> &mut A,
    <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A>,
{
    Ok(
        if let Some(model) = get_db::<A, I>(db, item, id, Some(model)).await? {
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
    A: ActiveModelTrait + ActiveModelBehavior + Send + Sync,
    I: Into<<<A::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>
        + Display
        + Clone
        + Sync
        + Send,
    <<A as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<A>,
{
    Ok(
        if let Some(model) = get_db::<A, I>(db, "guild", id, None).await? {
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
