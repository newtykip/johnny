use crate::create_migration;
use sea_orm_migration::prelude::*;

create_migration!(
    Guild,
    Table::create()
        // is autorole enabled?
        .col(
            ColumnDef::new(Guild::Autorole)
                .boolean()
                .not_null()
                .default(Value::Bool(Some(cfg!(debug_assertions)))),
        )
        // are sticky roles enabled?
        .col(
            ColumnDef::new(Guild::Sticky)
                .boolean()
                .not_null()
                .default(Value::Bool(Some(cfg!(debug_assertions)))),
        ),
    Autorole,
    Sticky
);
