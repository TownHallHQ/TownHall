pub mod config;
pub mod context;
pub mod graphql;
pub mod handlers;
pub mod services;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use axum::http::{header, HeaderValue, Method};
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::cors::CorsLayer;

use crate::context::Context;
use crate::graphql::schema::{MutationRoot, QueryRoot};

pub async fn start() -> Result<()> {
    let config = config::Config::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    let context = Context::new(&config).await?;
    let context = Arc::new(context);
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
        .await?;

    Ok(())
}
