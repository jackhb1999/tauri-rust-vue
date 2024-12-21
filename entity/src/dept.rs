//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize,Deserialize)]
#[sea_orm(table_name = "dept")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub dept_code: String,
    pub dept_name: String,
    pub parent_code: Option<String>,
    pub menus: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}