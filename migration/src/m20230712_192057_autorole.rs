use super::m20230711_163234_guild::Guild;
use crate::create_migration;
use sea_orm_migration::prelude::*;

create_migration!(
    Autorole,
    Table::create()
        // guild snowflake
        .col(ColumnDef::new(Autorole::GuildId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("autorole_guild_id_fk")
                .from_col(Autorole::GuildId)
                .to_tbl(Guild::Table)
                .to_col(Guild::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        // role snowflake
        .col(ColumnDef::new(Autorole::RoleId).text().not_null()),
    GuildId,
    RoleId,
);
