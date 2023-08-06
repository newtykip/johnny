use common::db::prelude::*;

pub async fn role_delete(role_id: &RoleId, pool: &Pool) -> Result<()> {
    // delete the associated autorole document
    delete!(Autorole, pool, RoleId | role_id.to_string())?;
    Ok(())
}
