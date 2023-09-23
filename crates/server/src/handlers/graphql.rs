use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::Extension;

use crate::context::SharedContext;
use crate::graphql::context::token::try_extract_token;
use crate::graphql::schema::GraphQLSchema;

pub async fn schema(
    Extension(schema): Extension<GraphQLSchema>,
    Extension(ctx): Extension<SharedContext>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    match try_extract_token(&ctx.services.auth, &headers) {
        Ok(maybe_token) => {
            if let Some(token) = maybe_token {
                req = req.data(token);
            }
        }
        Err(err) => {
            return GraphQLResponse::from(async_graphql::Response::new(err.to_string()));
        }
    }

    schema.execute(req).await.into()
}

pub async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
