use axum::extract::State;
use axum::Json;
use chrono::{Local};
use uuid::Uuid;
use crate::app::Db;
use crate::entity::app_error::AppError;
use crate::entity::book::Book;
use crate::entity::create_book::CreateBook;

pub struct BookService {}

impl BookService {
    pub async fn create_book(State(db): State<Db>, payload: Json<CreateBook>) -> Result<Json<Book>, AppError> {
        tracing::info!("request create user endpoint!");
        if has_invalid_params(payload.clone()) {
            return Err(AppError::InvalidParams);
        }

        let book = Book {
            id: Uuid::new_v4(),
            title: payload.title.clone().unwrap(),
            author: payload.author.clone().unwrap(),
            pages: payload.pages.clone().unwrap(),
            created_at: Local::now(),
            updated_at: Local::now()
        };

        db.write().unwrap().insert(book.id, book.clone());
        tracing::info!("Book with title: [{}] is created. ID: [{}]!", book.title, book.id);
        Ok(Json(book))
    }

    pub async fn get_books(State(db): State<Db>) -> Result<Json<Vec<Book>>, AppError> {
        tracing::info!("Request get all books endpoint!");
        let books = db.read().unwrap();
        let result = books.values().cloned().collect::<Vec<Book>>();
        tracing_result(result.len());
        Ok(Json(result))
    }
}

fn has_invalid_params(payload: Json<CreateBook>) -> bool {
    if payload.title.is_none() || payload.author.is_none() || payload.pages.is_none() { return true } return false
}

fn tracing_result(books_size: usize) {
    if books_size < 1 {
        tracing::info!("no book returned!");
    } else if books_size == 1 {
        tracing::info!("{} book returned!", books_size);
    } else {
        tracing::info!("{} books returned!", books_size);
    }
}