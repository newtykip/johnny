use crate::create_migration;
use sea_orm_migration::prelude::*;

create_migration!(User, Table::create(),);
