use once_cell::sync::Lazy;
use sea_orm_migration::prelude::*;

pub static TABLE: Lazy<TableCreateStatement> = Lazy::new(|| {
    Table::create()
        .table(User::Table)
        // user snowflake
        .col(ColumnDef::new(User::Id).text().not_null().primary_key())
        .to_owned()
});

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(TABLE.clone()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
