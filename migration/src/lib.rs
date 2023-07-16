use once_cell::sync::Lazy;
pub use sea_orm_migration::prelude::*;

mod m20230711_163234_guild;
mod m20230711_173402_user;
mod m20230712_192057_autorole;

pub static TABLES: Lazy<Vec<TableCreateStatement>> = Lazy::new(|| {
    vec![
        m20230711_163234_guild::TABLE.clone(),
        m20230711_173402_user::TABLE.clone(),
        m20230712_192057_autorole::TABLE.clone(),
    ]
});

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230711_163234_guild::Migration),
            Box::new(m20230711_173402_user::Migration),
            Box::new(m20230712_192057_autorole::Migration),
        ]
    }
}
