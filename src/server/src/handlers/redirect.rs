use std::str::FromStr;

use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;

use quicklink::link::model::ulid::Ulid;

use crate::context::Context;

pub async fn redirect(ctx: Extension<Context>, Path(hash): Path<String>) -> impl IntoResponse {
    let Ok(hash) = Ulid::from_str(&hash) else {
        return Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(Full::from("Invalid ULID provided for Link"))
        .unwrap()
        .into_response();
    };

    match ctx
        .services
        .link
        .find(Some(quicklink::link::repository::LinkFilter {
            ulid: Some(hash.clone()),
            ..Default::default()
        }))
        .await
    {
        Ok(records) => {
            if records.len() != 1 {
                tracing::error!("More than 1 record found for hash: {hash}. Ambiguous result.");
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Full::from("An internal error ocurred"))
                    .unwrap()
                    .into_response();
            }

            let link = records.get(0).unwrap();

            Redirect::to(link.original_url.as_ref()).into_response()
        }
        Err(err) => {
            tracing::error!(%err, "An error ocurred fetching link from database");

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from("An internal error ocurred"))
                .unwrap()
                .into_response()
        }
    }
}
