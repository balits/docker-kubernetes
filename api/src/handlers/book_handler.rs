use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::Book;

pub async fn index(State(pool): State<PgPool>) -> Json<Vec<Book>> {
    let q = "select * from books";
    let raw = sqlx::query_as::<_, Book>(q)
        .fetch_all(&pool)
        .await
        .context(format!("failed query on: {}", q))
        .unwrap();

    Json(raw)
}

pub async fn by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, (StatusCode, String)> {
    let book = sqlx::query_as::<_, Book>(
        "SELECT id, title, author_id, genre_id, publication_year, isbn, available FROM books WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    return match book {
        Ok(b) => Ok(Json(b)),
        Err(e) => Err(match e {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                format!("Book with {} id not found!", id),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Query failed: {:?}", e),
            ),
        }),
    };
}
