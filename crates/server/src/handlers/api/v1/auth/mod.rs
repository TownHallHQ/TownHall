pub mod login;

use axum::routing::get;
use axum::Router;

pub fn auth() -> Router {
    Router::new().route("/login", get(login::handler))
}
