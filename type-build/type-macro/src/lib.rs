#![allow(unstable)]
use proc_macro::TokenStream;
use std::fs;
use std::path::Path;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};


fn read_env_path() -> String {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.txt");
    let result = fs::read_to_string(&dest_path);
    result.unwrap_or_else(|_| "default_path".to_string())
}


#[proc_macro_attribute]
pub fn TS(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = read_env_path();
    match path.as_str() {
        "default_path" => {}
        _ => {
            let item_clone = item.clone();
            let input = parse_macro_input!(item_clone as DeriveInput);
            // 获取结构体名称
            let name = &input.ident;
            // 处理字段
            let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = input.data {
                fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;
                    quote! {
                #field_name: #field_type,
            }
                }).collect::<Vec<_>>()
            } else {
                Vec::new()
            };

            // 生成 TypeScript 接口代码
            let ts_code = quote! {
        interface #name {
            #(#fields)*
        }
    };
            let string = ts_code.to_string();
            // 获取文件名
            // let span = Span::call_site();
            // let filename = span.source_file().path().file_name()
            //     .and_then(|f| f.to_str())
            //     .unwrap_or(&name.to_string());

            let path_name = Path::new(&path).join(format!("{}.ts", attr.to_string()));

            match fs::exists(&path_name) {
                Ok(b) => {
                    if !b {
                        fs::write(&path_name, &string).expect("Unable to write file");
                    }
                }
                Err(_) => {}
            }

            // fs::write("D:\\gitee\\tauri-rust-vue\\output.txt",
            //           format!("input:{:?}\n\
            //   ATTR:{:?}\n\
            //   PATH:{:?}\n\
            //   ", string, attr.to_string(), path_name,
            //           ))
            //     .expect("TODO: panic message");
        }
    }
    item
}