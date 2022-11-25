use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;
use entity::{self, prelude::Link};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::context::SharedContext;

pub async fn redirect(
    ctx: Extension<SharedContext>,
    Path(hash): Path<String>,
) -> impl IntoResponse {
    let Ok(maybe_link) = Link::find()
        .filter(entity::link::Column::Hash.eq(hash))
        .one(&ctx.conn())
        .await else
    {
        return Response::builder()
           .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Full::from("An error ocurred fetching from database"))
            .unwrap()
            .into_response()
    };

    match maybe_link {
        Some(link) => Redirect::to(&link.original_url).into_response(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::from("Not Found"))
            .unwrap()
            .into_response(),
    }
}
