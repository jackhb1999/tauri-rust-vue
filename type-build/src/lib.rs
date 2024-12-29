use std::fs;
use std::path::Path;
pub use type_macro::TS;


/**
 * 构建生成路径
 * @param type_Path 生成路径（为绝对路径）
**/
pub fn build(type_Path: &str) {
    // 将 type_Path 路径保存到宏可以读取的地方
    let out_dir = std::env::var("OUT_DIR").unwrap();;
    let dest_path = Path::new(&out_dir).join("config.txt");
    fs::write(&dest_path, type_Path).unwrap();
}
