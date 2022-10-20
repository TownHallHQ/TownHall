use async_graphql::{Context, InputObject, Object, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::user::{User, UserError, UserErrorCode};

#[derive(Debug, Default, InputObject)]
pub struct UserCreateInput {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserCreate {
    user: Option<User>,
    error: Option<UserError>,
}

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    #[graphql(name = "userCreate")]
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> Result<UserCreate> {
        let context = ctx.data_unchecked::<SharedContext>();

        Ok(UserCreate {
            user: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }
}
