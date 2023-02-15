use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;

use crate::context::SharedContext;
use crate::shared::repository::Repository;

#[allow(dead_code)]
pub async fn redirect(
    ctx: Extension<SharedContext>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    if let Ok(Some(link)) = ctx.repositories.link.find_by_key(hash) {
        return Redirect::to(&link.original_url).into_response();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
        .into_response()
}
