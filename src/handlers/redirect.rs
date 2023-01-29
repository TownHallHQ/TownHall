use axum::extract::Path;

use axum::response::IntoResponse;
use axum::Extension;

use crate::context::SharedContext;

pub async fn redirect(
    _ctx: Extension<SharedContext>,
    Path(_hash): Path<String>,
) -> impl IntoResponse {
    todo!();
    // let Ok(maybe_link) = Link::find()
    //     .filter(entity::link::Column::Hash.eq(hash))
    //     .one(&ctx.conn())
    //     .await else
    // {
    //     return Response::builder()
    //        .status(StatusCode::INTERNAL_SERVER_ERROR)
    //         .body(Full::from("An error ocurred fetching from database"))
    //         .unwrap()
    //         .into_response()
    // };

    // match maybe_link {
    //     Some(link) => {
    //         info!(
    //             "Found link for: {} redirecting to: {}",
    //             link.hash, link.original_url
    //         );
    //         Redirect::to(&link.original_url).into_response()
    //     }
    //     None => {
    //         info!("No link found");
    //         Response::builder()
    //             .status(StatusCode::NOT_FOUND)
    //             .body(Full::from("Not Found"))
    //             .unwrap()
    //             .into_response()
    //     }
    // }
}
