use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Book {
    id: i32,
    pub title: String,
    pub author_id: i32,
    pub genre_id: i32,
    pub publication_year: Option<i32>,
    pub isbn: Option<String>,
    pub available: bool,
}
