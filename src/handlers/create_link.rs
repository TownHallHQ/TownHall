use axum::http::Uri;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::Deserialize;

use crate::context::Context;
use crate::entities::link::Link;

#[derive(Debug, Deserialize)]
pub struct CreateLinkInput {
    url: String,
}

pub async fn create_link(
    ctx: Extension<Context>,
    Json(payload): Json<CreateLinkInput>,
) -> impl IntoResponse {
    let original_url = payload.url.parse::<Uri>().unwrap();
    let conn = ctx.conn();
    let link = Link::new(conn, original_url).await;

    Json(link)
}
