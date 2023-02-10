use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct CreateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pages: Option<i64>
}