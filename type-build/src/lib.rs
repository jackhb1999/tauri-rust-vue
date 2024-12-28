use std::fs;
use std::time::SystemTime;
pub use type_macro::TS;

pub fn build() {
    println!("Hello, world!");
    println!("type build");
    fs::write("D:\\gitee\\tauri-rust-vue\\output.txt",
              format!("执行成功{:?}", SystemTime::now()))
        .expect("TODO: panic message");
}
