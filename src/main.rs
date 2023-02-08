mod entity;
mod service;
mod controller;

use axum::{Json, Router};
use axum::routing::{get, post};
use serde_json::{Value};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::entity::create_book::CreateBook;
use crate::controller::book_controller::BookController;
use crate::entity::app_error::AppError;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "axum_api=debug".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/books", post(create_book))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8090));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

async fn hello() -> Json<Value> {
    tracing::info!("Request Hello Endpoint!");
    BookController::hello().await
}

async fn health() -> Json<Value> {
    tracing::info!("Request Health Endpoint!");
    BookController::health().await
}

async fn create_book(payload: Json<CreateBook>) -> Result<Json<Value>, AppError> {
    tracing::info!("Request Create User Endpoint!");
    BookController::create(payload).await
}
