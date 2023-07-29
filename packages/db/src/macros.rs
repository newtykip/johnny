/// Find a model by id
#[macro_export]
macro_rules! find_one {
    ($entity:ident, $db:expr, $id:expr) => {
        if let Ok(Some(model)) = $crate::$entity::Entity::find_by_id($id.to_string())
            .one($db)
            .await
        {
            Ok(model)
        } else {
            Err(common::prelude::eyre!("failed to find model"))
        }
    };
}

/// Update a model
#[macro_export]
macro_rules! update {
    ($entity:ident, $db:expr, $id:expr, $($prop:ident => $value:expr),+) => {
        {
            if let Some(model) = find_one!($entity, $db, $id) {
                let mut active = model.into_active_model();

                $(
                    active.$prop = Set($value);
                )*

                active.update($db).await
            } else {
                Err(common::prelude::eyre!("failed to find model"))
            }
        }
    };

    ($model:expr, $db:expr, $($prop:ident => $value:expr),+) => {
        {
            let mut active = $model.clone().into_active_model();

            $(
                active.$prop = Set($value);
            )*

            active.update($db).await
        }
    };
}
