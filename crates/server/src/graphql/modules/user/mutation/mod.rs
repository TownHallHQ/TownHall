mod token_create;
mod user_register;

use async_graphql::{Context, Object, Result};

use self::token_create::TokenCreate;
use self::user_register::{UserRegister, UserRegisterInput};

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
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        input: UserRegisterInput,
    ) -> Result<UserRegister> {
        UserRegister::exec(ctx, input).await
    }
}
