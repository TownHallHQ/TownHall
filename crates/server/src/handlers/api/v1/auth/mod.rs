pub mod login;

use axum::routing::get;
use axum::Router;

pub fn auth() -> Router {
    Router::new().nest("/auth", Router::new().route("/login", get(login::handler)))
}
