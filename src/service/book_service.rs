use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{Local};
use uuid::Uuid;
use crate::app::Db;
use crate::entity::app_error::AppError;
use crate::entity::book::Book;
use crate::entity::create_book::CreateBook;
use crate::entity::update_book::UpdateBook;

pub struct BookService {}

impl BookService {
    pub async fn create(State(db): State<Db>, payload: Json<CreateBook>) -> Response {
        tracing::info!("endpoint: create user!");
        if has_invalid_params_on_create(payload.clone()) {
            return AppError::InvalidParams.into_response();
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
        tracing::info!("book with title: [{}] is created. ID: [{}]!", book.title, book.id);
        return Response::builder()
            .status(StatusCode::CREATED)
            .body(Json(book).into_response())
            .unwrap()
            .into_response();
    }

    pub async fn get_all(State(db): State<Db>) -> Response {
        tracing::info!("endpoint: get all books!");
        let books = db.read().unwrap();
        let result = books.values().cloned().collect::<Vec<Book>>();
        tracing_result(result.len());
        return Response::builder()
            .status(StatusCode::OK)
            .body(Json(result).into_response())
            .unwrap()
            .into_response();
    }

    pub async fn get(State(db): State<Db>, Path(id): Path<Uuid>) -> Response {
        tracing::info!("endpoint: get book by id {}", &id);
        let books = db.read().unwrap();
        let book = books.get(&id);
        match book {
            None => AppError::NotFound.into_response(),
            Some(book) => Response::builder()
                .status(StatusCode::OK)
                .body(Json(book.clone()).into_response())
                .unwrap()
                .into_response()
        }
    }

    pub async fn delete(State(db): State<Db>, Path(id): Path<Uuid>) -> Response {
        tracing::info!("endpoint: delete book by id {}", &id);
        if db.write().unwrap().remove(&id).is_some() {
            Response::builder().status(StatusCode::NO_CONTENT).body(Json(()).into_response()).unwrap().into_response()
        } else {
            AppError::NotFound.into_response()
        }
    }

    pub async fn update(State(db): State<Db>, Path(id): Path<Uuid>, Json(payload): Json<UpdateBook>) -> Result<Json<Book>, AppError> {
        tracing::info!("endpoint: update book by id {}", &id);
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