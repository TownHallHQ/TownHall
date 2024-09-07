use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use reqwest::Client;

use token_create::{TokenCreateTokenCreateError, TokenCreateTokenCreateToken};

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Debug",
    schema_path = "schema.json",
    query_path = "src/modules/auth/token_create/TokenCreate.gql"
)]
pub struct TokenCreate {
    pub token: Option<TokenCreateTokenCreateToken>,
    pub error: Option<TokenCreateTokenCreateError>,
}

pub async fn token_create(client: &Client, email: String, password: String) -> TokenCreate {
    let variables = token_create::Variables { email, password };
    let res = post_graphql::<TokenCreate, _>(client, "/graphql", variables)
        .await
        .unwrap();
    let data = res.data.unwrap().token_create;

    TokenCreate {
        token: data.token,
        error: data.error,
    }
}
