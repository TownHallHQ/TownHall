mod me;

use async_graphql::{Context, Object, Result};

use crate::graphql::guard::AuthenticationGuard;

use self::me::Me;

#[derive(Debug, Default)]
pub struct AuthQueryRoot;

#[Object]
impl AuthQueryRoot {
    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        Me::exec(ctx).await
    }
}
