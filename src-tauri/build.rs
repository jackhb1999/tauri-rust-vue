use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    type_build::build("D:\\gitee\\tauri-rust-vue\\src\\type");
    tauri_build::build()
}
