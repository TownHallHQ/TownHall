mod user_create;

use async_graphql::{Context, Object, Result};

use self::user_create::{UserCreate, UserCreateInput};
use crate::modules::auth::graphql::token_create::TokenCreate;

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
