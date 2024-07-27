pub mod api;
pub mod graphql;

use std::sync::Arc;

use axum::http::{header, HeaderValue, Method};
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::cors::CorsLayer;

use crate::context::Context;
use crate::graphql::schema::GraphQLSchema;

pub fn router(context: Arc<Context>, schema: GraphQLSchema) -> Router {
    let api = api::api();

    Router::new()
        .route("/graphql", get(graphql::playground).post(graphql::schema))
        .nest("/api", api)
        .layer(Extension(context))
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST]),
        )
}
