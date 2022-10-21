mod me;

use async_graphql::{Context, Object, Result};

use self::me::Me;

#[derive(Debug, Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    #[graphql(name = "me")]
    async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        Me::exec(ctx).await
    }
}
