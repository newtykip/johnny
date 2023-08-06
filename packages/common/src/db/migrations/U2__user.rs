use crate::use_iden;
use barrel::{backend, types, Migration};

use_iden!(As; User);

#[allow(dead_code)]
pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table(User::Table, |t| {
        t.add_column(User::Id, types::text().primary(true));
    });

    #[cfg(mysql)]
    return m.make::<backend::MySql>();

    #[cfg(postgres)]
    return m.make::<backend::Pg>();

    #[cfg(sqlite)]
    return m.make::<backend::Sqlite>();
}
