use sea_orm::DatabaseConnection;
use tauri::State;
use handle::vo::UserInfo::UserInfo;

mod vo;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn login(name: &str, pass: &str, db: State<'_, DatabaseConnection>) -> Result<UserInfo, ()> {
    println!("Logging in...{}", pass);
    println!("Logging in...{:?}", db);
    let info = handle::UserHandle::login_handle(name, pass, db).await.expect("登录失败！");
    // 登录
    // Ok(serde_json::to_string(&info).unwrap())
    Ok(info)
}


#[tauri::command]
async fn updatePassword(usercode: &str, oldPW: &str, newPW: &str, db: State<'_, DatabaseConnection>) -> Result<String, ()> {
    handle::UserHandle::change_password_handle(usercode, oldPW, newPW, db).await.expect("修改失败！");
    Ok("修改成功！".to_string())
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
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async move {
    //         let db = handle::ConnectionHandle::establish_connection().await.unwrap();
    let db = tauri::async_runtime::block_on(handle::ConnectionHandle::establish_connection()).unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![greet,login,updatePassword,
            getGoodsList,getUserList,getRoleList])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // });
}


