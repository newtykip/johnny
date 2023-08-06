pub use super::{entities, QUERY_BUILDER};
pub use paste::paste;
pub use sea_query::{Cond, Expr, Query};
pub use sea_query_binder::SqlxValues;
pub use sqlx::{query_as_with, query_with};
pub use strum::IntoEnumIterator;

/// Make an SQL query with the given arguments.
#[macro_export]
macro_rules! query_with {
    ($sql:expr, $values:expr) => {{
        $crate::db::macros::query_with::<_, $crate::db::macros::SqlxValues>(&$sql, $values)
    }};
}

/// Make a SQL query, with the given arguments, that is mapped to a concrete type using FromRow.
#[macro_export]
macro_rules! query_as_with {
    ($sql:expr, $values:expr, $type:ty) => {{
        $crate::db::macros::query_as_with::<_, $type, $crate::db::macros::SqlxValues>(
            &$sql, $values,
        )
    }};
}

/// Format the given values into a vector of SimpleExpr
#[macro_export]
macro_rules! values {
    ($($value:expr),+) => {{
        vec![$($value.into(),)*]
    }}
}

#[macro_export]
macro_rules! use_iden {
    ($($entity:ident),+) => {
        $crate::db::macros::paste! {
            $(
                use $crate::db::entities::[<$entity:lower>]::Iden;
            )*
        };
    };
    (As; $($entity:ident),+) => {
        $crate::db::macros::paste! {
            $(
                use $crate::db::entities::[<$entity:lower>]::Iden as $entity;
            )*
        }
    };
}

// todo: extract some logic into a function
#[macro_export(local_inner_macros)]
macro_rules! select {
    (@ $entity:ident) => {
        use_iden!($entity);
        use $crate::db::macros::IntoEnumIterator;
    };
    ($entity:ident, $pool:expr, $($where:ident | $eq:expr),+) => {{
        select!(@ $entity);

        let (sql, values) = $crate::db::macros::Query::select()
            .from(Iden::Table)
            .columns(Iden::iter().collect::<Vec<_>>())
            .cond_where(
                $crate::db::macros::Cond::all()
                    $(.add($crate::db::macros::Expr::col(Iden::$where).eq($eq)))*
            )
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_as_with!(sql, values, $crate::db::macros::paste! { $crate::db::entities::[<$entity:lower>]::$entity }).fetch_one($pool).await.ok()
    }};
    (Many, $entity:ident, $pool:expr) => {{
        select!(@ $entity);

        let (sql, values) = $crate::db::macros::Query::select()
            .from(Iden::Table)
            .columns(Iden::iter().collect::<Vec<_>>())
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_as_with!(sql, values, $crate::db::macros::paste! { $crate::db::entities::[<$entity:lower>]::$entity }).fetch_all($pool).await.ok()
    }};
    (Many, $entity:ident, $pool:expr, $($where:ident | $eq:expr),+) => {{
        select!(@ $entity);

        let (sql, values) = $crate::db::macros::Query::select()
            .from(Iden::Table)
            .columns(Iden::iter().collect::<Vec<_>>())
            .cond_where(
                $crate::db::macros::Cond::all()
                    $(.add($crate::db::macros::Expr::col(Iden::$where).eq($eq)))*
            )
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_as_with!(sql, values, $crate::db::macros::paste! { $crate::db::entities::[<$entity:lower>]::$entity }).fetch_all($pool).await.ok()
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! update {
    ($entity:ident, $pool:expr, $($where:ident | $eq:expr),+; $($col:ident => $value:expr),+) => {{
        use_iden!($entity);

        let (sql, values) = $crate::db::macros::Query::update()
            .table(Iden::Table)
            .values([$((Iden::$col, $value.into()),)*])
            .cond_where(
                $crate::db::macros::Cond::all()
                    $(.add($crate::db::macros::Expr::col(Iden::$where).eq($eq)))*
            )
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_with!(sql, values).execute($pool).await
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! delete {
    ($entity:ident, $pool:expr, $($where:ident | $eq:expr),+) => {{
        use_iden!($entity);

        let (sql, values) = $crate::db::macros::Query::delete()
            .from_table(Iden::Table)
            .cond_where(
                $crate::db::macros::Cond::all()
                    $(.add($crate::db::macros::Expr::col(Iden::$where).eq($eq)))*
            )
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_with!(sql, values).execute($pool).await
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! insert {
    ($entity:ident, $pool:expr, $($col:ident => $value:expr),+) => {{
        use_iden!($entity);

        let (sql, values) = $crate::db::macros::Query::insert()
            .into_table(Iden::Table)
            .columns([$(Iden::$col,)*])
            .values_panic([$($value.into(),)*])
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_with!(sql, values).execute($pool).await
    }};

    (Many, $entity:ident, $pool:expr, $($col:ident => $values:expr),+) => {{
        use_iden!($entity);

        let (sql, values) = $crate::db::macros::Query::insert()
            .into_table(Iden::Table)
            .columns([$(Iden::$col,)*])
            $(.values_panic($values.iter().map(|x| x.into()).collect::<Vec<_>>()))*
            .build_sqlx($crate::db::macros::QUERY_BUILDER);

        query_with!(sql, values).execute($pool).await
    }};
}
