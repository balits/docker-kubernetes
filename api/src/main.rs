use std::net::SocketAddr;

use axum::{routing::get, Router};
use library_api::{init, shutdown};
use tracing::debug;

mod handlers;
mod models;

use handlers::book_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = init::create_pool().await.unwrap();

    let app = Router::new()
        .route("/books", get(book_handler::index))
        .route("/books/:id", get(book_handler::by_id))
        .with_state(pool);

    let addr: SocketAddr = "127.0.0.1:7676".parse().unwrap();

    debug!("Listening on {}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::gracefull())
        .await?;

    Ok(())
}
