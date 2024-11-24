use axum::body::Body;
use axum::Router;
use axum::routing::get;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
}