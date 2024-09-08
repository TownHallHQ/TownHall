use anyhow::{Result, anyhow};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use pxid::Pxid;

use townhall_types::user::Email;
use user_register::{UserRegisterUserRegisterError, UserRegisterUserRegisterUser};

pub use crate::auth::user_register::user_register::UserRegisterInput;

use crate::DateTime;

use super::AuthClient;

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Debug",
    schema_path = "schema.json",
    query_path = "src/modules/auth/user_register/UserRegister.gql"
)]
pub struct UserRegister {
    pub user: Option<UserRegisterUserRegisterUser>,
    pub error: Option<UserRegisterUserRegisterError>,
}

pub async fn user_register(auth_client: &AuthClient, input: UserRegisterInput) -> Result<UserRegister> {
    let variables = user_register::Variables { input };
    let url = auth_client.domain.join("/graphql")?;
    let res = post_graphql::<UserRegister, _>(&auth_client.client, url, variables)
        .await
        .map_err(|err| anyhow!("Failed to register user. {err}"))?;
    let data = res.data.unwrap().user_register;

    Ok(UserRegister {
        user: data.user,
        error: data.error,
    })
}
