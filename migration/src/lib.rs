pub use sea_orm_migration::prelude::*;

mod m20230711_163234_create_guild;
mod m20230711_173402_create_user;

pub use m20230711_163234_create_guild::TABLE as GUILD_TABLE;
pub use m20230711_173402_create_user::TABLE as USER_TABLE;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230711_163234_create_guild::Migration),
            Box::new(m20230711_173402_create_user::Migration),
        ]
    }
}
