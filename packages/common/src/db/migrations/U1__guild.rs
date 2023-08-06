use crate::use_iden;
use barrel::{backend, types, Migration};

const DEFAULT: bool = cfg!(debug_assertions);

use_iden!(As; Guild);

#[allow(dead_code)]
pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table(Guild::Table, |t| {
        t.add_column(Guild::Id, types::text().primary(true));
        t.add_column(Guild::Autorole, types::boolean().default(DEFAULT));
        t.add_column(Guild::Sticky, types::boolean().default(DEFAULT));
    });

    #[cfg(mysql)]
    return m.make::<backend::MySql>();

    #[cfg(postgres)]
    return m.make::<backend::Pg>();

    #[cfg(sqlite)]
    return m.make::<backend::Sqlite>();
}
