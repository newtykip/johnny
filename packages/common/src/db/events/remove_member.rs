use crate::db::{entities::member::Iden, prelude::*};

pub async fn remove_member(member: &Member, pool: &Pool) -> Result<()> {
    let (sql, values) = Query::delete()
        .from_table(Iden::Table)
        .and_where(Expr::col(Iden::UserId).eq(member.user.id.to_string()))
        .and_where(Expr::col(Iden::GuildId).eq(member.guild_id.to_string()))
        .build_sqlx(QUERY_BUILDER);

    query_with!(sql, values).execute(pool).await?;

    Ok(())
}
