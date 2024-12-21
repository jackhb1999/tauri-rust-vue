use std::future;
use std::future::ready;
use std::io::ErrorKind;
use std::task::ready;
use anyhow::bail;
use sea_orm::{Database, Set};
use sea_orm::prelude::*;
use tauri::async_runtime::block_on;
use tauri::State;

use entity::user;
use crate::vo;
use crate::vo::UserInfo::UserInfo;

// 用于登录
pub async fn login_handle(code: &str, pass: &str, db: State<'_, DatabaseConnection>) -> anyhow::Result<UserInfo> {
    let conn = db.inner();
    println!("获取连接");

    let users: Vec<user::Model> = entity::user::Entity::find()
        .filter(user::Column::Username.eq(code).and(user::Column::Password.eq(pass)))
        .all(conn)
        .await?;

    // 如果存在返回成功
    if !users.is_empty() {
        println!("{}登录成功！", code);
        if let Some(user) = users.first() {
            let mut info = UserInfo::new(&user);
            let conn_x = conn.clone();
            // 查询部门
            let x = if let Some(dept_code) = &user.dept_code {
                entity::dept::Entity::find()
                    .filter(entity::dept::Column::DeptCode.eq(dept_code)).one(&conn_x)
            } else {
                entity::dept::Entity::find().one(&conn_x)
            };
            // 查询角色
            let comm_y = conn.clone();
            let y = if let Some(role_code) = &user.role_code {
                entity::role::Entity::find()
                    .filter(entity::role::Column::RoleCode.eq(role_code)).one(&comm_y)
            } else {
                entity::role::Entity::find().one(&comm_y)
            };
            let (depts, roles) = tokio::join!(x,y);
            if let Some(deptModel) = depts? {
                info.depts = vec![deptModel];
            }
            if let Some(roleModel) = roles? {
                info.roles = vec![roleModel];
            }
            // 查询菜单  部门和角色的查询可异步进行，查到后查菜单同步
            let menus = entity::menu::Entity::find().all(conn).await?;
            info.menus = menus;

            println!("57:{:?}", &info);
            return Ok(info);
        }
    }
    bail!("没有查到指定用户{}",code)
}

pub async fn register_handle(user_vo: user::Model, db: State<'_, DatabaseConnection>) {
    let conn = db.inner();
    // 如果失败则保存入库
    let user_model = user::ActiveModel {
        username: Set(user_vo.username),
        password: Set(user_vo.password),
        ..Default::default()
    };
    if let Ok(_user) = user_model.save(conn).await {
        println!("Admin user created");
    } else {
        println!("Failed to create user");
    }
}


pub fn change_password_handle(oldPW: &str, newPW: &str, db: State<'_, DatabaseConnection>) -> anyhow::Result<()> {
    let conn = db.inner();

    return Ok(());
}