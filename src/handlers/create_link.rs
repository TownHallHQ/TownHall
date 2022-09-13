use axum::Json;
use axum::http::Uri;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLinkInput {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkOutput {
    id: u64,
    shortened_url: String,
}

pub async fn create_link(Json(payload): Json<CreateLinkInput>) -> impl IntoResponse {
    let original_url = payload.url.parse::<Uri>().unwrap();

    Json(CreateLinkOutput {
        id: 1,
        shortened_url: format!("https://{}/{}", "shortened", original_url),
    })
}
