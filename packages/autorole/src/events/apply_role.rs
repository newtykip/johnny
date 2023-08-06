use common::db::prelude::*;
use common::event::*;
use std::collections::HashSet;

pub async fn apply_role(ctx: &Context, member: &mut Member, pool: &Pool) -> Result<()> {
    let guild = select!(Guild, pool, Id | member.guild_id.to_string()).unwrap();

    if guild.autorole {
        if let Some(autoroles) =
            select!(Many, Autorole, pool, GuildId | member.guild_id.to_string())
        {
            let roles = autoroles
                .iter()
                .map(|x| RoleId(x.role_id.parse().unwrap()))
                .collect::<HashSet<_>>();

            for id in roles {
                member.add_role(&ctx.http, id).await?;
            }
        }
    }

    Ok(())
}
