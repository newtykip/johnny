pub use super::{DB, QUERY_BUILDER};
pub use crate::{delete, insert, prelude::*, query_as_with, query_with, select, update, values};
pub use sea_query::{Expr, Query, Value};
pub use sea_query_binder::SqlxBinder;
use sqlx::Pool as SqlxPool;
pub use sqlx::{query, query_as};

pub type Pool = SqlxPool<DB>;

// generate snowflakes
use crate::EPOCH;
#[cfg(any(autorole, sticky))]
use {once_cell::sync::Lazy, rustflake::Snowflake};

#[cfg(any(autorole, sticky))]
static mut SNOWFLAKE: Lazy<Snowflake> = Lazy::new(|| Snowflake::new(EPOCH, 0, 0));

#[cfg(any(autorole, sticky))]
pub fn generate_id() -> String {
    unsafe { SNOWFLAKE.generate() }.to_string()
}
