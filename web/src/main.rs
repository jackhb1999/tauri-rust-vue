use axum::body::{Body, Bytes};
use axum::extract::{Multipart, Path, Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::{map_request, map_response};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use axum::handler::Handler;
use http_body_util::Full;
use std::iter::Map;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

// 携带状态 必须实现 Clone trait
#[derive(Clone)]
struct AppState {
    var1: Arc<AtomicUsize>,
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let tracing = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracing).expect("Failed to set global default");

    let app = Router::new()
        .route("/", get(root))
        .route("/test/{id}", get(test_id))
        .route("/testquery", post(test_query))
        .route("/testfile", post(test_file))
        .route("/users", post(create_user))
        .route("/users2", post(create_user2))
        .route("/useState", get(useState))
        // 携带状态
        .with_state(AppState {
            var1: Arc::new(AtomicUsize::new(0)),
        })
        // 中间件 在请求前和请求后执行
        .route_layer(map_request(test_map_request))
        .route_layer(map_response(test_map_response))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::limit::RequestBodyLimitLayer::new(64))
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(
            handle_panic,
        ));
    let nest = Router::new().nest(
        // 统一前缀
        "/nest",
        Router::new()
            .route("/", get(root))
            .route("/test/{id}", get(test_id)),
    );
    // 合并 Router
    let merge = app.merge(nest);
    // web 错误处理
    let fallback = merge
        .fallback(|| async move { "404" })
        .method_not_allowed_fallback(|| async move { "405" });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, fallback).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// 返回参数要实现 Serialize trait
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

// 接收参数要实现 Deserialize trait
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// 接收 json 参数
async fn create_user(Json(create_user): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        name: create_user.username,
    };
    (StatusCode::CREATED, Json(user))
}

struct ApiResponse<T: Serialize> {
    status: StatusCode,
    content: T,
    message: String,
}

// 实现 IntoResponse trait，才可以用于返回
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            // Body::from 实现 From trait, 可以从 &str, String, Vec<u8> 等类型转换为 Body
            .body(Body::from(
                serde_json::to_string(&self.content).unwrap_or("".to_string()),
            ))
            .unwrap()
    }
}

async fn create_user2(Json(create_user): Json<CreateUser>) -> ApiResponse<User> {
    let user = User {
        id: 1,
        name: create_user.username,
    };
    ApiResponse {
        status: StatusCode::CREATED,
        content: user,
        message: "User created successfully".to_string(),
    }
}

// 从路径中获取参数
async fn test_id(Path(id): Path<u64>) -> String {
    format!("User id: {}", id)
}

#[derive(Deserialize, Debug)]
struct TestQuery {
    name: String,
    age: u64,
    // 如果没有提供参数，使用默认值
    #[serde(default = "default_other")]
    other: String,
}

fn default_other() -> String {
    "default".to_string()
}

// 从查询参数中获取参数 localhost:3000/testquery?name=John&age=20
async fn test_query(Query(query): Query<TestQuery>) -> String {
    format!(
        "User name: {}, age: {},other:{}",
        query.name, query.age, query.other
    )
}

// 从表单中获取参数
// POST localhost:3000/testfile
// Content-Type: multipart/form-data
async fn test_file(mut part: Multipart) {
    while let Some(field) = part.next_field().await.unwrap() {
        println!("field:{:?}", field.name());
        println!("field_type:{:?}", field.content_type());
    }
}

async fn useState(State(state): State<AppState>) {
    println!(
        "var1: {}",
        state.var1.load(std::sync::atomic::Ordering::Relaxed)
    );
    state
        .var1
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

async fn test_map_request(mut req: Request) -> Request {
    info!("request method:{:?},uri:{:?}", req.method(), req.uri());
    req.extensions_mut().insert(String::from("Test"));
    println!("req extensions:{:?}", req.extensions().get::<String>());
    req
}

async fn test_map_response(res: Response) -> Response {
    res
}

fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> Response<Full<Bytes>> {
    error!("panic: {:?}", err);
    let err_str = r#"
    {
        "status": 500,
        "message": "Internal server error",
        "data": null
    }
    "#;
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header("Content-Type", "application/json")
        .body(Full::from(err_str))
        .unwrap()
}
