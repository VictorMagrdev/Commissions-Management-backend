use axum::http::{header::*, Method};
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use katheryne_backend::api::router::create_router;
use katheryne_backend::infrastructure::data::db_context::surreal_context::connect_db;

#[tokio::main]
async fn main() {
    connect_db().await.unwrap();
    let cors:CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET,Method::PUT, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app:Router = create_router().layer(cors);

    println!("🚀 Server started successfully");
    let listener:TcpListener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}