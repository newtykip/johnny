use crate::db::{entities::guild::Iden, prelude::*};

pub async fn create_guild(guild: &Guild, pool: &Pool) -> Result<()> {
    let (sql, values) = Query::insert()
        .into_table(Iden::Table)
        .columns([Iden::Id])
        .values(values![guild.id.to_string()])?
        .build_sqlx(QUERY_BUILDER);

    query_with!(sql, values).execute(pool).await?;

    Ok(())
}
