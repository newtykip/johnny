pub use helpers::TABLES;
pub use sea_orm_migration::prelude::*;

mod helpers;
pub mod m20230711_163234_guild;
pub mod m20230711_173402_user;
pub mod m20230712_192057_autorole;
pub mod m20230716_231443_member;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230711_163234_guild::Migration),
            Box::new(m20230711_173402_user::Migration),
            Box::new(m20230712_192057_autorole::Migration),
            Box::new(m20230716_231443_member::Migration),
        ]
    }
}
