use super::{m20230711_163234_guild::Guild, m20230711_173402_user::User};
use crate::create_migration;
use sea_orm_migration::prelude::*;

create_migration!(
    Sticky,
    Table::create()
        // guild snowflake
        .col(ColumnDef::new(Sticky::GuildId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("sticky_guild_id_fk")
                .from_col(Sticky::GuildId)
                .to_tbl(Guild::Table)
                .to_col(Guild::Id)
                .on_delete(ForeignKeyAction::Cascade)
        )
        // user snowflake
        .col(ColumnDef::new(Sticky::UserId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("sticky_user_id_fk")
                .from_col(Sticky::UserId)
                .to_tbl(User::Table)
                .to_col(User::Id)
                .on_delete(ForeignKeyAction::Cascade)
        )
        // todo: make it an array in postgres
        // role snowflake
        .col(ColumnDef::new(Sticky::RoleId).text().not_null())
        // guild, user and role ids must all be unique
        .index(
            Index::create()
                .name("Sticky_guild_id_user_id_idx")
                .col(Sticky::GuildId)
                .col(Sticky::UserId)
                .col(Sticky::RoleId)
                .unique()
        ),
    GuildId,
    UserId,
    RoleId
);
