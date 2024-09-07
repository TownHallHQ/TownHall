use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use pxid::Pxid;
use reqwest::Client;

use townhall_types::user::Email;
use user_register::{UserRegisterUserRegisterError, UserRegisterUserRegisterUser};

pub use crate::auth::user_register::user_register::UserRegisterInput;

use crate::DateTime;

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

pub async fn user_register(client: &Client, input: UserRegisterInput) -> UserRegister {
    let variables = user_register::Variables { input };
    let res = post_graphql::<UserRegister, _>(client, "/graphql", variables)
        .await
        .unwrap();
    let data = res.data.unwrap().user_register;

    UserRegister {
        user: data.user,
        error: data.error,
    }
}
