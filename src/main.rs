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
use crate::modules::link::service::CreateLinkDto;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::new();
    // let addr = SocketAddr::from((config.server_host, config.server_port));
    let context = Context::new(&config)
        .await
        .expect("Failed to build context.");

    let new_link = CreateLinkDto {
        original_url: String::from("http://whizzes.com"),
        owner_id: Some(String::from("6CKGl7zNyYxVGPnB")),
    };

    let link_id = context.services.link.create(new_link);

    println!("{link_id}");

    // let new_user = CreateUserDto {
    //     name: String::from("Dave"),
    //     last_name: String::from("Arenas"),
    //     email: String::from("dave136@gmail.com"),
    // };

    // let new_user2 = CreateUserDto {
    //     name: String::from("Dave2"),
    //     last_name: String::from("Arenas2"),
    //     email: String::from("dave1362@gmail.com"),
    // };

    // context.services.user.create(new_user);
    // context.services.user.create(new_user2);

    // let all = context.services.user.get_all();
    // println!("{:?}", all);
    // context.services.user.get(user_id);
}
