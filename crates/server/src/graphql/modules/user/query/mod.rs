mod me;
mod users;

use async_graphql::{Context, Object, Result};
use pxid::graphql::Pxid;

use crate::graphql::guard::AuthenticationGuard;

use self::{
    me::Me,
    users::{Users, UsersConnection},
};

#[derive(Debug, Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        Me::exec(ctx).await
    }

    async fn user(
        &self,
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
        id: Option<Pxid>,
        email: Option<String>,
        username: Option<String>,
    ) -> Result<UsersConnection> {
        Users::exec(ctx, after, before, first, last, id, email, username).await
    }
}
