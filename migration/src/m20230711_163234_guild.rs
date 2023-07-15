use once_cell::sync::Lazy;
use sea_orm_migration::prelude::*;

pub static TABLE: Lazy<TableCreateStatement> = Lazy::new(|| {
    Table::create()
        .table(Guild::Table)
        // guild snowflake
        .col(ColumnDef::new(Guild::Id).text().not_null().primary_key())
        // is autorole enabled?
        .col(
            ColumnDef::new(Guild::Autorole)
                .boolean()
                .not_null()
                .default(Value::Bool(Some(false))),
        )
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
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Guild {
    Table,
    Id,
    Autorole,
}
