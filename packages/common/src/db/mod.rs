pub mod entities;
#[cfg(db_events)]
pub mod events;
pub mod macros;
pub mod migrate;
pub mod prelude;

pub use entities::*;

// ensure that a database driver has been selected
#[cfg(not(any(mysql, postgres, sqlite)))]
compile_error!("please enable one of \"postgres\", \"mysql\", or \"sqlite\"");

#[cfg(mysql)]
pub type DB = sqlx::MySql;
#[cfg(postgres)]
pub type DB = sqlx::Postgres;
#[cfg(sqlite)]
pub type DB = sqlx::Sqlite;

#[cfg(mysql)]
pub const QUERY_BUILDER: sea_query::MySqlQueryBuilder = sea_query::MysqlQueryBuilder;
#[cfg(postgres)]
pub const QUERY_BUILDER: sea_query::PostgresQueryBuilder = sea_query::PostgresQueryBuilder;
#[cfg(sqlite)]
pub const QUERY_BUILDER: sea_query::SqliteQueryBuilder = sea_query::SqliteQueryBuilder;
