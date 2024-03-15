use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Genre {
    id: i32,
    pub name: String,
    pub description: Option<String>,
}
