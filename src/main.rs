mod context;
// mod graphql;
mod handlers;
mod modules;

// use std::net::SocketAddr;
// use std::sync::Arc;

// use async_graphql::{EmptySubscription, Schema};
// use axum::http::{header, HeaderValue, Method};
// use axum::routing::get;
// use axum::{Extension, Router};
// use tower_http::cors::CorsLayer;
// use tracing::info;

use crate::context::Config;
use crate::context::Context;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new();
    // let addr = SocketAddr::from((config.server_host, config.server_port));
    let context = Context::new(&config)
        .await
        .expect("Failed to build context.");

    context.services.user.create();
    context.services.user.get();
}
