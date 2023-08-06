use common::db::prelude::*;
use common::event::*;

pub async fn add_roles(ctx: &Context, member: &mut Member, pool: &Pool) -> Result<()> {
    if let Some(sticky_roles) = select!(
        Many,
        Sticky,
        pool,
        UserId | member.user.id.to_string(),
        GuildId | member.guild_id.to_string()
    ) {
        for role in sticky_roles {
            member
                .add_role(&ctx.http, RoleId(role.role_id.parse()?))
                .await?;
        }
    }

    Ok(())
}
