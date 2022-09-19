use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;

pub async fn redirect(ctx: Extension<Context>, Path(hash): Path<String>) -> impl IntoResponse {
    let conn = ctx.conn();

    if let Some(link) = Link::get_by_hash(conn, &hash).await {
        return Redirect::to(&link.original_url).into_response();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::from("not found"))
        .unwrap()
        .into_response()
}
