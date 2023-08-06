use crate::db::{entities::guild::Iden, prelude::*};

pub async fn remove_guild(guild: &UnavailableGuild, pool: &Pool) -> Result<()> {
    let (sql, values) = Query::delete()
        .from_table(Iden::Table)
        .and_where(Expr::col(Iden::Id).eq(guild.id.to_string()))
        .build_sqlx(QUERY_BUILDER);

    query_with!(sql, values).execute(pool).await?;

    Ok(())
}
