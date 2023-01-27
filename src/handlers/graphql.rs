use std::str::FromStr;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::ServerError;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::Extension;

use crate::modules::auth::service::Token;
use crate::modules::GraphQLSchema;

pub async fn schema(
    req: GraphQLRequest,
    Extension(schema): Extension<GraphQLSchema>,
    headers: HeaderMap,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(maybe_token) = headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(Token::from_str).ok())
    {
        match maybe_token {
            Ok(token) => req = req.data(token),
            Err(err) => {
                return GraphQLResponse::from(async_graphql::Response::from_errors(vec![
                    ServerError::new(err.to_string(), None),
                ]));
            }
        }
    }

    schema.execute(req).await.into()
}

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
