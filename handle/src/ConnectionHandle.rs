use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> anyhow::Result<DatabaseConnection> {
    let db_url = "postgres://postgres:fackpg@localhost/test";
    let conn = Database::connect(db_url).await
        .expect("connection established: 连接出错");
    println!("已创建连接");
    Ok(conn)
}