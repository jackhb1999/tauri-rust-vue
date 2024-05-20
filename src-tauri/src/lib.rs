// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login(name: &str,pass:&str) -> String {
    format!("Hello, {}! You've been greeted from Rust!{}", name,pass)
}

#[tauri::command]
fn getGoodsList() -> Vec<Goods> {
    let mut vec:Vec<Goods> = Vec::new();
    let goods1 = Goods{
        id:1,
        introduce:String::from("web应用开发实战书"),
        title:"react书".to_string(),
        user_id:12
    };
    vec.push(goods1);
    vec.push(Goods{
        id:2,
        introduce:"什么设计模式".to_string(),
        title:"设计模式".to_string(),
        user_id:13
    });
    return vec;
}


#[derive(serde::Serialize)]
struct Goods{
    id:i32,
    introduce:String,
    title:String,
    user_id:i32,

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,login,getGoodsList])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
