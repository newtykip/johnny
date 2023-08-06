use crate::db::{member, prelude::*, user};

pub async fn remove_user(user: &User, pool: &Pool) -> Result<()> {
    // are there any member rows left?
    let (sql, values) = Query::select()
        .from(member::Iden::Table)
        .column(member::Iden::Id)
        .and_where(Expr::col(member::Iden::UserId).eq(user.id.to_string()))
        .limit(1)
        .build_sqlx(QUERY_BUILDER);

    let no_rows = query_as_with!(sql, values, member::Member)
        .fetch_all(pool)
        .await?
        .len()
        == 0;

    if no_rows {
        let (sql, values) = Query::delete()
            .from_table(user::Iden::Table)
            .and_where(Expr::col(user::Iden::Id).eq(user.id.to_string()))
            .build_sqlx(QUERY_BUILDER);

        query_with!(sql, values).execute(pool).await?;
    }

    Ok(())
}
