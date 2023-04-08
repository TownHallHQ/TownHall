use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;

use crate::context::Context;

pub async fn redirect(ctx: Extension<Context>, Path(hash): Path<String>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::from("not found"))
        .unwrap()
        .into_response()
}
