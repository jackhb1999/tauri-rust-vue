use proc_macro::TokenStream;
use std::fs;
use std::time::SystemTime;
use syn::{parse_macro_input, DeriveInput};

struct Path {
    pub path: String,
}

const PATHD: &str = "MY_PARAM";

fn read_env_var_in_crate_a() -> String {
    std::env::var(PATHD).unwrap_or_else(|_| "default_value".to_string())
}

#[proc_macro_attribute]
pub fn TS(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let input = parse_macro_input!(item_clone as DeriveInput);
    let path = read_env_var_in_crate_a();
    fs::write("D:\\gitee\\tauri-rust-vue\\output.txt",
              format!("input:{:?}\n\
              attr:{:?}\n\
              PATH:{:?}\n\
              time:{:?}", input, attr, std::env::var(PATHD), SystemTime::now()
              ))
        .expect("TODO: panic message");
    item
}