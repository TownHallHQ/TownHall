mod user_create;

use async_graphql::{Context, Object, Result};

use self::user_create::{UserCreate, UserCreateInput};

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    #[graphql(name = "userCreate")]
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> Result<UserCreate> {
        UserCreate::exec(ctx, input).await
    }
}
