pub mod auth;

use axum::Router;

pub fn v1() -> Router {
    Router::new().nest("/auth", auth::auth())
}
