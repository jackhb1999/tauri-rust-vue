use rusqlite::Connection;
use std::io;
use std::path::Path;
// use web::run;

fn main() -> Result<(), rusqlite::Error> {
    // run().await;
    let db_path = "test.db";
    let conn = connect_or_create_database(db_path)?;
    println!("成功连接到数据库: {}", db_path);
    // 在这里可以继续执行其他数据库操作
    Ok(())
}

fn connect_or_create_database(path: &str) -> Result<Connection, rusqlite::Error> {
    let db_path = Path::new(path);
    if db_path.exists() {
        // 如果数据库文件存在，直接连接
        println!("数据库文件存在，正在连接...");
        Connection::open(db_path)
    } else {
        // 如果数据库文件不存在，创建并连接
        println!("数据库文件不存在，正在创建并连接...");
        let conn = Connection::open(db_path)?;
        // 这里可以添加初始化数据库的 SQL 语句，例如创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS test_table (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(conn)
    }
}
