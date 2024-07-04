use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Debug",
    schema_path = "schema.json",
    query_path = "src/modules/auth/token_create/TokenCreate.gql"
)]
pub struct TokenCreate;

pub async fn token_create(email: String, password: String) -> token_create::TokenCreateTokenCreate {
    let client = Client::builder().build().unwrap();
    let variables = token_create::Variables { email, password };
    let res = post_graphql::<TokenCreate, _>(&client, "http://127.0.0.1:7878/graphql", variables)
        .await
        .unwrap();

    res.data.unwrap().token_create
}
