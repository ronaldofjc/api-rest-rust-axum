use axum::Json;
use serde_json::{json, Value};
use crate::entity::app_error::AppError;
use crate::entity::create_book::CreateBook;
use crate::service::book_service::BookService;

pub struct BookController {}

impl BookController {
    pub async fn hello() -> Json<Value> {
        Json(json!({ "message": "API Rust with Axum is working!!!" }))
    }

    pub async fn health() -> Json<Value> {
        Json(json!({ "status":"UP" }))
    }

    pub async fn create(payload: Json<CreateBook>) -> Result<Json<Value>, AppError> {
        match BookService::create(payload).await {
            Ok(book) => {
                tracing::info!("Livro cadastrado com sucesso. ID: {}", book.id.to_string());
                Ok(Json(json!(book)))
            },
            Err(err) => Err(err)
        }
    }
}