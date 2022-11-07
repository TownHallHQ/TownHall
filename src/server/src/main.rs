mod config;
mod context;
mod graphql;
mod handlers;
mod services;

use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use axum::http::{header, HeaderValue, Method};
use axum::routing::{get, post};
use axum::{Extension, Router};
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::new();
    let addr = SocketAddr::from((config.server_host, config.server_port));
    let context = context::Context::new(&config)
        .await
        .expect("Failed to build context.");
    let context = Arc::new(context);
    let schema = Schema::build(
        graphql::QueryRoot::default(),
        graphql::MutationRoot::default(),
        EmptySubscription,
    )
    .data(Arc::clone(&context))
    .finish();
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

    info!("Configuration {:#?}", config);
    info!("Listening on {:#?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
