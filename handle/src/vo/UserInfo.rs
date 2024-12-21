use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use entity::{dept, menu, role, user};


#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize)]
pub struct UserInfo {
    pub user_code: String,
    pub username: String,
    pub depts: Vec<dept::Model>,
    pub roles: Vec<role::Model>,
    pub menus: Vec<menu::Model>,
}

impl UserInfo {
    pub fn new(user: &user::Model) -> Self {
        let mut user_info: UserInfo = UserInfo {
            user_code: user.user_code.clone(),
            username: user.username.clone(),
            depts: vec![],
            roles: vec![],
            menus: vec![],
        };
        user_info
    }
}