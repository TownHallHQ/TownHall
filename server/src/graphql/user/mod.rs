use async_graphql::{Context, EmptySubscription, Enum, Object, Schema, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::services::auth::Token;

pub struct UserQueryRoot;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct UserError {
    code: UserErrorCode,
    message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct User {
    id: u32,
    name: String,
    last_name: String,
    email: String,
    hash: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct Me {
    user: User,
    error: UserError,
}

#[Object]
impl UserQueryRoot {
    async fn me<'a>(&self, ctx: &'a Context<'_>) -> Option<Me> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();

            return Some(claims.uid);
        }

        None
    }
}
