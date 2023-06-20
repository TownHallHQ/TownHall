mod config;
mod context;
mod graphql;
mod handlers;
mod services;

use std::net::SocketAddr;
use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use axum::http::{header, HeaderValue, Method};
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::cors::CorsLayer;

use crate::context::Context;
use crate::graphql::modules::{MutationRoot, QueryRoot};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    let context = Context::shared(config).await;
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(Arc::clone(&context))
    .finish();
    let app = Router::new()
        .route(
            "/graphql",
            get(handlers::graphql::playground).post(handlers::graphql::schema),
        )
        .layer(Extension(context))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST]),
        );

    tracing::info!("GraphQL Playground available on http://{}/graphql", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
