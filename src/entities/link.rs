use axum::http::Uri;
use chrono::{prelude::*, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Link {
    pub id: Uuid,
    pub hash: String,
    pub original_url: String,
    pub expires_at: DateTime<Utc>,
}

impl Link {
    pub async fn new(conn: PgPool, original_url: Uri) -> Self {
        let hash = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>();
        let utc: DateTime<Utc> = Utc::now() + Duration::days(10);
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

    pub async fn get_by_hash(conn: PgPool, hash: &str) -> Option<Self> {
        // This actually fails silently because we are treating every possible
        // error as a none blindly.
        //
        // This must be improved in the future.
        sqlx::query_as(
            r#"
            SELECT
                *
            FROM
                links
            WHERE
                "links".hash = $1
                AND NOW() < "links".expires_at
            "#,
        )
        .bind(hash)
        .fetch_one(&conn)
        .await
        .ok()
    }
}
