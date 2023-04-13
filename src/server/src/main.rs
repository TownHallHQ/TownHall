mod config;
mod context;
mod graphql;
mod handlers;

use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    let app = Router::new().route("/:hash", get(handlers::redirect::redirect));

    tracing::info!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
