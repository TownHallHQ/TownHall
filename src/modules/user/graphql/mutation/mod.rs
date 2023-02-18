mod token_create;
mod user_create;

use async_graphql::{Context, Object, Result};

use self::token_create::TokenCreate;
use self::user_create::{UserCreate, UserCreateInput};

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
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> Result<UserCreate> {
        UserCreate::exec(ctx, input).await
    }
}
