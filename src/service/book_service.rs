use axum::Json;
use chrono::Utc;
use uuid::Uuid;
use crate::entity::app_error::AppError;
use crate::entity::book::Book;
use crate::entity::create_book::CreateBook;

pub struct BookService {}

impl BookService {
    pub async fn create(payload: Json<CreateBook>) -> Result<Book, AppError> {
        if payload.title.clone().is_none() || payload.author.clone().is_none() {
            return Err(AppError::InvalidParams);
        }

        Ok(Book {
            id: Uuid::new_v4(),
            title: payload.title.clone().unwrap(),
            author: payload.author.clone().unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now()
        })
    }
}