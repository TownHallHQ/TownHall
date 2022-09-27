mod config;
mod context;
mod graphql;
mod handlers;
mod services;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::http::{header, HeaderValue, Method};
use axum::routing::{get, post};
use axum::{Extension, Router};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    let context = context::Context::new(&config)
        .await
        .expect("Failed to build context.");
    let schema = Schema::new(graphql::QueryRoot, EmptyMutation, EmptySubscription);
    let app = Router::new()
        .route("/:hash", get(handlers::redirect::redirect))
        .route("/new", post(handlers::create_link::create_link))
        .route(
            "/graphql",
            get(handlers::graphql::playground).post(handlers::graphql::schema),
        )
        .layer(Extension(context))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin(config.cors_allow_origin.parse::<HeaderValue>().unwrap())
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
