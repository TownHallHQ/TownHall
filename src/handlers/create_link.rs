use axum::http::Uri;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use entity::prelude::*;
use serde::Deserialize;

use crate::context::Context;

#[derive(Debug, Deserialize)]
pub struct CreateLinkInput {
    url: String,
}

pub async fn create_link(
    ctx: Extension<Context>,
    Json(payload): Json<CreateLinkInput>,
) -> impl IntoResponse {
    let original_url = payload.url.parse::<Uri>().unwrap();
    Json(link)
}
