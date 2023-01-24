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

use modules::user::service::CreateUserDto;

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

    let new_user = CreateUserDto {
        name: String::from("Dave"),
        last_name: String::from("Arenas"),
        email: String::from("dave136@gmail.com"),
    };

    let user_id = context.services.user.create(new_user);
    context.services.user.get(user_id);
}
