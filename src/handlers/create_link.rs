use anyhow::{Error, Result};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::{prelude::*, Duration};
use entity::Link;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::context::Context;

#[derive(Debug, Deserialize)]
pub struct CreateLinkInput {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkOutput {
    id: i32,
    hash: String,
    original_url: String,
    expires_at: DateTime<Utc>,
}

pub async fn create_link(
    ctx: Extension<Context>,
    Json(payload): Json<CreateLinkInput>,
) -> impl IntoResponse {
    let original_url = parse_url(&payload.url).unwrap();
    let expires_at: DateTime<Utc> = Utc::now() + Duration::days(10);
    let naive_expires_at = expires_at.naive_utc();
    let link = Link {
        id: NotSet,
        hash: Set(create_hash()),
        original_url: Set(payload.url),
        expires_at: Set(naive_expires_at),
        created_at: NotSet,
        updated_at: NotSet,
    };
    let link = link.save(&ctx.conn()).await.unwrap();

    Json(CreateLinkOutput {
        id: link.id.unwrap(),
        hash: link.hash.unwrap(),
        original_url: link.original_url.unwrap(),
        expires_at,
    })
}

fn create_hash() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
}

fn parse_url(raw: &str) -> Result<Url> {
    Url::parse(raw).map_err(|_| Error::msg(format!("Provided URL is not valid: {}", raw)))
}

#[cfg(test)]
mod test {
    use super::parse_url;

    #[test]
    fn validates_urls() {
        let example = parse_url("https://www.example.com");
        let arbitrary_text = parse_url("arbitrary_text");

        assert!(example.is_ok());
        assert!(arbitrary_text.is_err());
    }
}
