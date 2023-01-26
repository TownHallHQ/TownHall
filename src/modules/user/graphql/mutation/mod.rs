mod user_create;

use self::user_create::{UserCreate, UserCreateInput};
use async_graphql::{Context, Object, Result};

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> Result<UserCreate> {
        UserCreate::exec(ctx, input).await
    }
}
