use tauri::utils::html::parse;

mod vo;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login(name: &str, pass: &str) -> String {
    println!("Logging in...{}",pass);
    handle::LogonHandle::handle(name, pass).expect("登录失败！");
    // 登录
    format!("Hello, {}! You've been greeted from Rust!{}", name, pass)
}

#[tauri::command]
fn getGoodsList() -> Vec<Goods> {
    let mut vec: Vec<Goods> = Vec::new();
    let goods1 = Goods {
        id: 1,
        introduce: String::from("web应用开发实战书"),
        title: "react书".to_string(),
        user_id: 12,
    };
    vec.push(goods1);
    for i in 2..12 {
        vec.push(Goods {
            id: i,
            introduce: "什么设计模式".to_string(),
            title: "设计模式".to_string(),
            user_id: 13,
        });
    }
    return vec;
}

#[tauri::command]
fn getUserList() -> Vec<User> {
    let mut vec: Vec<User> = Vec::new();
    let role = RoleKV {
        role_id: 1,
        role_name: "管理员".to_string(),
    };
    let mut roles = Vec::new();
    roles.push(role);
    let user = User {
        id: 1,
        nick_name: "小红".to_string(),
        user_name: "xiaohong".to_string(),
        role: roles,
    };
    vec.push(user);
    return vec;
}

#[tauri::command]
fn getRoleList() -> Vec<Role> {
    let mut vec: Vec<Role> = Vec::new();
    let role0 = Role {
        role_id: 1,
        role_name: "管理员".to_string(),
        authority: Vec::from([1]),
    };
    let role1 = Role {
        role_id: 2,
        role_name: "普通用户".to_string(),
        authority: vec![1],
    };
    vec.push(role0);
    vec.push(role1);

    return vec;
}


#[derive(serde::Serialize)]
struct Goods {
    id: i32,
    introduce: String,
    title: String,
    user_id: i32,

}

#[derive(serde::Serialize)]
struct User {
    id: i32,
    nick_name: String,
    user_name: String,
    role: Vec<RoleKV>,

}

#[derive(serde::Serialize)]
struct RoleKV {
    role_id: i32,
    role_name: String,
}

#[derive(serde::Serialize)]
struct Role {
    role_id: i32,
    role_name: String,
    authority: Vec<i32>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,login,getGoodsList,getUserList,getRoleList])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
