use crate::m20230711_163234_guild::Guild;
use once_cell::sync::Lazy;
use sea_orm_migration::prelude::*;

pub static TABLE: Lazy<TableCreateStatement> = Lazy::new(|| {
    Table::create()
        .table(Autorole::Table)
        // unique snowflake
        .col(ColumnDef::new(Autorole::Id).text().not_null().primary_key())
        // guild snowflake
        .col(ColumnDef::new(Autorole::GuildId).text().not_null())
        // role snowflake
        .col(ColumnDef::new(Autorole::RoleId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("autorole_guild_id_fk")
                .from_col(Autorole::GuildId)
                .to_tbl(Guild::Table)
                .to_col(Guild::Id)
                .on_delete(ForeignKeyAction::Cascade),
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
            .drop_table(Table::drop().table(Autorole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Autorole {
    Table,
    Id,
    GuildId,
    RoleId,
}
