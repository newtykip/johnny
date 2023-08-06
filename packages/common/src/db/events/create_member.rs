use crate::db::{entities::member::Iden, prelude::*};

pub async fn create_member(member: &Member, pool: &Pool) -> Result<()> {
    let (sql, values) = Query::insert()
        .into_table(Iden::Table)
        .columns([Iden::Id, Iden::UserId, Iden::GuildId])
        .values(values![
            generate_id(),
            member.user.id.to_string(),
            member.guild_id.to_string()
        ])?
        .build_sqlx(QUERY_BUILDER);

    query_with!(sql, values).execute(pool).await?;

    Ok(())
}
