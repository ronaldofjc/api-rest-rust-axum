use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateBook {
    pub title: Option<String>,
    pub author: Option<String>
}