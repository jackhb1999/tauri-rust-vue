use std::fs;
use std::time::SystemTime;

fn main() {
    const PATHD: &str = "MY_PARAM";
    std::env::set_var(PATHD,"my_param_value");
    fs::write("D:\\gitee\\tauri-rust-vue\\output1.txt",
              format!("
              PATH:{:?}\n\
              time:{:?}", std::env::var(PATHD), SystemTime::now()
              ));
    tauri_build::build()
}
