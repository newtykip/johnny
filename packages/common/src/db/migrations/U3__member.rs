use crate::use_iden;
use barrel::{backend, types, Migration};

use_iden!(As; Member);

#[allow(dead_code)]
pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table(Member::Table, |t| {
        t.add_column(Member::Id, types::text().primary(true));
        t.add_column(Member::UserId, types::text().nullable(false));
        t.add_column(Member::GuildId, types::text().nullable(false));
    });

    #[cfg(mysql)]
    return m.make::<backend::MySql>();

    #[cfg(postgres)]
    return m.make::<backend::Pg>();

    #[cfg(sqlite)]
    return m.make::<backend::Sqlite>();
}
