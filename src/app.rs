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

    let db = Db::new(RwLock::new(HashMap::new()));

    Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/ping", get(ping))
        .route("/books", post(BookService::create).get(BookService::get_all))
        .route("/books/:id", get(BookService::get).delete(BookService::delete).put(BookService::update))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(db)
}

pub type Db = Arc<RwLock<HashMap<Uuid, Book>>>;

async fn hello() -> Json<Value> {
    tracing::info!("request hello endpoint!");
    Json(json!({ "message": "API Rust with Axum is working!!!" }))
}

async fn health() -> Json<Value> {
    tracing::info!("request health endpoint!");
    Json(json!({ "status":"UP" }))
}

async fn ping() -> Json<Value> {
    tracing::info!("request ping endpoint!");
    Json(json!({ "message":"pong" }))
}