use async_graphql::{Context, EmptySubscription, Object, Schema};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::services::auth::Token;

pub struct UserQueryRoot;

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

        None
    }
}
