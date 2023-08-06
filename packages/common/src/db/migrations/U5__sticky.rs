use crate::use_iden;
use barrel::{backend, types, Migration};

use_iden!(As; Sticky);

#[allow(dead_code)]
pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table(Sticky::Table, |t| {
        t.add_column(Sticky::Id, types::text().primary(true));
        t.add_column(Sticky::UserId, types::text().nullable(false));
        t.add_column(Sticky::GuildId, types::text().nullable(false));
        t.add_column(Sticky::RoleId, types::text().nullable(false));
    });

    #[cfg(mysql)]
    return m.make::<backend::MySql>();

    #[cfg(postgres)]
    return m.make::<backend::Pg>();

    #[cfg(sqlite)]
    return m.make::<backend::Sqlite>();
}
