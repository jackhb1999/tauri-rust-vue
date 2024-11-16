use sea_orm::{Database, Set};
use sea_orm::prelude::*;
use serde_json::json;
use entity::user;

pub fn handle(name: &str, pass: &str) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = "postgres://postgres:mima@localhost/test";

            let conn = Database::connect(db_url).await
                .expect("Failed to connect to database");

            let users: Vec<entity::user::Model> = entity::user::Entity::find()
                .filter(user::Column::Username.eq(name))
                .all(&conn)
                .await?;

            // 如果存在返回成功
            if !users.is_empty() {
                println!("Admin user already exists");
                return Ok(());
            }

            // 如果失败则保存入库

            let user_model = entity::user::ActiveModel{
                username: Set(name.to_string()),
                password: Set(pass.to_string()),
                ..Default::default()
            };
            if let Ok(_user) = user_model.save(&conn).await {
                println!("Admin user created");
            } else {
                println!("Failed to create user");
            }


            Ok::<(), anyhow::Error>(())
        })?;


    Ok(())
}