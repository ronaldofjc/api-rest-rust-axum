use std::collections::HashMap;
use axum::{Json, Router};
use std::sync::{Arc, RwLock};
use axum::routing::{get, post};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use crate::entity::book::Book;
use crate::service::book_service::BookService;

pub async fn create_app() -> Router {
    let db = Db::default();

    Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/books", post(BookService::create_book).get(BookService::get_books))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db)
}

pub type Db = Arc<RwLock<HashMap<Uuid, Book>>>;

async fn hello() -> Json<Value> {
    tracing::info!("Request Hello Endpoint!");
    Json(json!({ "message": "API Rust with Axum is working!!!" }))
}

async fn health() -> Json<Value> {
    tracing::info!("Request Health Endpoint!");
    Json(json!({ "status":"UP" }))
}