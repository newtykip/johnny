pub use crate::entity::user::*;
use crate::{get_db, prelude::*};
use async_trait::async_trait;
use common::prelude::*;

const ITEM: &str = "user";

#[async_trait]
impl DB<Model> for User {
    async fn db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db::<ActiveModel, String>(
            db,
            ITEM,
            &self.id.to_string(),
            ActiveModel {
                id: Set(self.id.to_string()),
                ..Default::default()
            },
        )
        .await
    }
}

#[async_trait]
impl DB<Model> for UserId {
    async fn db(&self, db: &DatabaseConnection) -> Result<Option<Model>> {
        get_db::<ActiveModel, String>(
            db,
            ITEM,
            &self.to_string(),
            ActiveModel {
                id: Set(self.to_string()),
                ..Default::default()
            },
        )
        .await
    }
}
