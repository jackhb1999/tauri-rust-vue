//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize,Deserialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_code: String,
    pub role_name: String,
    pub menus: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
