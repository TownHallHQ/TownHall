mod config;
mod context;
mod entities;
mod handlers;

use axum::routing::{get, post};
use axum::{Extension, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::new();
    let context = context::Context::new(config).await;

    context.bootstrap().await;

    let app = Router::new()
        .route("/:hash", get(handlers::redirect::redirect))
        .route("/new", post(handlers::create_link::create_link))
        .layer(Extension(context));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
