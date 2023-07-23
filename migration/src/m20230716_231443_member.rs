use super::{m20230711_163234_guild::Guild, m20230711_173402_user::User};
use crate::create_migration;
use sea_orm_migration::prelude::*;

create_migration!(
    Member,
    Table::create()
        // guild snowflake
        .col(ColumnDef::new(Member::GuildId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("member_guild_id_fk")
                .from_col(Member::GuildId)
                .to_tbl(Guild::Table)
                .to_col(Guild::Id)
                .on_delete(ForeignKeyAction::Cascade)
        )
        // user snowflake
        .col(ColumnDef::new(Member::UserId).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("member_user_id_fk")
                .from_col(Member::UserId)
                .to_tbl(User::Table)
                .to_col(User::Id)
                .on_delete(ForeignKeyAction::Cascade)
        )
        // guild and user id must both be unique
        .index(
            Index::create()
                .name("member_guild_id_user_id_idx")
                .col(Member::GuildId)
                .col(Member::UserId)
                .unique()
        ),
    GuildId,
    UserId,
);
