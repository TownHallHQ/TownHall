pub mod token_create;

use async_graphql::{Context, Object, Result};

use self::token_create::TokenCreate;

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn token_create(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> Result<TokenCreate> {
        TokenCreate::exec(ctx, email, password).await
    }
}
