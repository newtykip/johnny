use crate::db::{entities::user::Iden, prelude::*};

pub async fn create_user(user: &User, pool: &Pool) -> Result<()> {
    let (sql, values) = Query::insert()
        .into_table(Iden::Table)
        .columns([Iden::Id])
        .values(values![user.id.to_string()])?
        .build_sqlx(QUERY_BUILDER);

    query_with!(sql, values).execute(pool).await?;

    Ok(())
}
