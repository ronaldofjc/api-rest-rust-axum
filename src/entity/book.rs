use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub pages: i64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>
}