mod routes;

use axum::Router;
use axum::routing::get;
use crate::routes::create_routes;

pub async fn run() {
    let app = create_routes();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}