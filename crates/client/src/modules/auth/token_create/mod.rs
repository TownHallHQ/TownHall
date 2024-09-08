use anyhow::{Result, anyhow};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;

use token_create::{TokenCreateTokenCreateError, TokenCreateTokenCreateToken};

use super::AuthClient;

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

pub async fn token_create(auth_client: &AuthClient, email: String, password: String) -> Result<TokenCreate> {
    let variables = token_create::Variables { email, password };
    let url = auth_client.domain.join("/graphql")?;
    let res = post_graphql::<TokenCreate, _>(&auth_client.client, url, variables)
        .await
        .map_err(|err| anyhow!("Failed to create token. {err}"))?;
    let data = res.data.unwrap().token_create;

    Ok(TokenCreate {
        token: data.token,
        error: data.error,
    })
}
