use axum::http::Uri;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::context::Context;

#[derive(Debug, Deserialize)]
pub struct CreateLinkInput {
    url: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Link {
    id: Uuid,
    hash: String,
    original_url: String,
    expires_at: DateTime<Utc>,
}

impl Link {
    pub async fn new(conn: PgPool, original_url: Uri) -> Self {
        let hash = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>();
        let utc: DateTime<Utc> = Utc::now();

        let link = sqlx::query_as(
            r#"INSERT INTO links (
                hash,
                original_url,
                expires_at
            ) VALUES (
                $1,
                $2,
                $3
            )
            RETURNING *
            "#,
        )
        .bind(hash)
        .bind(original_url.to_string())
        .bind(utc)
        .fetch_one(&conn)
        .await
        .unwrap();

        link
    }
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
