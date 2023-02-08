use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub status: String,
}

impl Error {
    pub fn new(message: String, status: String) -> Self {
        Self { message, status }
    }
}