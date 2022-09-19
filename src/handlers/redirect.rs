use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;
use entity::Link;
use sea_orm::ColumnTrait;

use crate::context::Context;

pub async fn redirect(ctx: Extension<Context>, Path(hash): Path<String>) -> impl IntoResponse {
    let link: Option<Link> = Link::find()
        .filter(entity::link::Column::Hash.eq(hash))
        .one(&ctx.conn())
        .await
        .unwrap();

    // if let Some(link) = Link::get_by_hash(conn, &hash).await {
    //     return Redirect::to(&link.original_url).into_response();
    // }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::from("not found"))
        .unwrap()
        .into_response()
}
