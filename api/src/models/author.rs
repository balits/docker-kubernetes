use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Author {
    id: i32,
    pub name: String,
    pub birth_year: Option<i32>,
    pub biography: Option<String>,
}
