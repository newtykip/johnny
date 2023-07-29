use super::*;
use once_cell::sync::Lazy;

pub static TABLES: Lazy<Vec<TableCreateStatement>> = Lazy::new(|| {
    vec![
        m20230711_163234_guild::TABLE.clone(),
        m20230711_173402_user::TABLE.clone(),
        m20230712_192057_autorole::TABLE.clone(),
        m20230716_231443_member::TABLE.clone(),
        m20230727_210614_sticky::TABLE.clone(),
    ]
});

#[macro_export]
macro_rules! create_migration {
    ($name: ident, $table: expr, $($element: ident),*) => {
        use once_cell::sync::Lazy;

        #[derive(Iden)]
        pub enum $name {
            Table,
            Id,
            $($element),*
        }

        pub static TABLE: Lazy<TableCreateStatement> = Lazy::new(|| $table.table($name::Table).col(ColumnDef::new($name::Id).text().not_null().primary_key()).to_owned());

        #[derive(DeriveMigrationName)]
        pub struct Migration;

        #[async_trait::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                manager.create_table(TABLE.clone()).await
            }

            async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                manager.drop_table(Table::drop().table($name::Table).to_owned()).await
            }
        }
    };
}
