use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Local};
use uuid::Uuid;
use crate::app::Db;
use crate::entity::app_error::AppError;
use crate::entity::book::Book;
use crate::entity::create_book::CreateBook;
use crate::entity::update_book::UpdateBook;

pub struct BookService {}

impl BookService {
    pub async fn create(State(db): State<Db>, payload: Json<CreateBook>) -> Result<Json<Book>, AppError> {
        tracing::info!("Endpoint: create user!");
        if has_invalid_params_on_create(payload.clone()) {
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

    pub async fn get_all(State(db): State<Db>) -> Result<Json<Vec<Book>>, AppError> {
        tracing::info!("Endpoint: Get all books!");
        let books = db.read().unwrap();
        let result = books.values().cloned().collect::<Vec<Book>>();
        tracing_result(result.len());
        Ok(Json(result))
    }

    pub async fn get(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<Json<Book>, AppError> {
        tracing::info!("Endpoint: Get book by id {}", &id);
        let books = db.read().unwrap();
        let book = books.get(&id);
        match book {
            None => Err(AppError::NotFound),
            Some(book) => Ok(Json(book.clone()))
        }
    }

    pub async fn delete(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, AppError> {
        tracing::info!("Endpoint: Delete book by id {}", &id);
        if db.write().unwrap().remove(&id).is_some() {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn update(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<UpdateBook>) -> Result<Json<Book>, AppError> {
        tracing::info!("Endpoint: Update book by id {}", &id);
        if has_invalid_params_on_update(payload.clone()) {
            return Err(AppError::InvalidParams);
        }

        let mut book = db.read().unwrap()
            .get(&id)
            .cloned()
            .ok_or(AppError::NotFound)?;

            book.title = payload.title.clone().unwrap();
            book.title = payload.title.clone().unwrap();
            book.author = payload.author.clone().unwrap();
            book.pages = payload.pages.clone().unwrap();
            book.updated_at = Local::now();

            db.write().unwrap().insert(book.id, book.clone());
            Ok(Json(book))
    }
}

fn has_invalid_params_on_create(payload: Json<CreateBook>) -> bool {
    if payload.title.is_none() || payload.author.is_none() || payload.pages.is_none() { return true } return false
}

fn has_invalid_params_on_update(payload: UpdateBook) -> bool {
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