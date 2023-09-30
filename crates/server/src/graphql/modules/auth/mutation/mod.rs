mod token_create;

use async_graphql::{Context, Object, Result};

use crate::graphql::scalars::email::Email;

use self::token_create::TokenCreate;

#[derive(Debug, Default)]
pub struct AuthMutationRoot;

#[Object]
impl AuthMutationRoot {
    async fn token_create(
        &self,
        ctx: &Context<'_>,
        email: Email,
        password: String,
    ) -> Result<TokenCreate> {
        TokenCreate::exec(ctx, email, password).await
    }
}
