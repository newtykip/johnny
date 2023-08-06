use common::db::prelude::*;

// todo: only on sticky
// todo: fix
pub async fn save_roles(member: &Member, pool: &Pool) -> Result<()> {
    let guild = select!(Guild, pool, Id | member.guild_id.to_string()).unwrap();

    if guild.sticky {
        // collect data
        let ids = vec![generate_id(); member.roles.len()];
        let guild_ids = vec![member.guild_id.to_string(); member.roles.len()];
        let user_ids = vec![member.user.id.to_string(); member.roles.len()];
        let role_ids = member
            .roles
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        // insert it
        insert!(Many, Sticky, pool, Id => ids, GuildId => guild_ids, UserId => user_ids, RoleId => role_ids)?;
    }

    Ok(())
}
