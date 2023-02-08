use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::response::Response;
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::entity::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    InvalidParams,
    InternalServerError
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::InvalidParams => (StatusCode::BAD_REQUEST, "Parâmetros inválidos na requisição")
        };
        (status, Json(json!(Error::new(err_msg.to_string(), status.to_string())))).into_response()
    }
}