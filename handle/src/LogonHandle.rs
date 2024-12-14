use sea_orm::{Database, Set};
use sea_orm::prelude::*;
use tauri::State;
use entity::user;



pub async fn handle(name: &str, pass: &str, db: State<'_, DatabaseConnection>) -> anyhow::Result<()> {
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async move {
            let conn = db.inner();
            println!("获取连接");

            let users: Vec<entity::user::Model> = entity::user::Entity::find()
                .filter(user::Column::Username.eq(name))
                .all(conn)
                .await?;

            // 如果存在返回成功
            if !users.is_empty() {
                println!("Admin user already exists");
                return Ok(());
            }

            // 如果失败则保存入库

            let user_model = user::ActiveModel{
                username: Set(name.to_string()),
                password: Set(pass.to_string()),
                ..Default::default()
            };
            if let Ok(_user) = user_model.save(conn).await {
                println!("Admin user created");
            } else {
                println!("Failed to create user");
            }


            Ok::<(), anyhow::Error>(())
        // })?;


    // Ok(())
}



pub fn change_password_handle(oldPW: &str, newPW: &str,db: State<'_, DatabaseConnection>) -> anyhow::Result<()> {
    let conn = db.inner();

    return Ok(())
}