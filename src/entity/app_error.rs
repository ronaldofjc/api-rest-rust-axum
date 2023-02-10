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

const INVALID_PARAMS_MSG: &str = "Invalid params on request";
const SERVER_ERROR_MSG: &str = "an internal server error occurred";

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => {
                tracing::error!("{}", SERVER_ERROR_MSG);
                (StatusCode::INTERNAL_SERVER_ERROR, SERVER_ERROR_MSG)
            },
            Self::InvalidParams => {
                tracing::warn!("{}", INVALID_PARAMS_MSG);
                (StatusCode::BAD_REQUEST, INVALID_PARAMS_MSG)
            }
        };
        (status, Json(json!(Error::new(err_msg.to_string(), status.to_string())))).into_response()
    }
}