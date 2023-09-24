mod token_create;
mod user_follow;
mod user_register;
mod user_unfollow;
mod user_update;

use async_graphql::{Context, Object, Result};
use pxid::graphql::Pxid;

use self::token_create::TokenCreate;
use self::user_follow::UserFollow;
use self::user_register::{UserRegister, UserRegisterInput};
use self::user_unfollow::UserUnfollow;
use self::user_update::{UserUpdate, UserUpdateInput};

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

    async fn user_update(
        &self,
        ctx: &Context<'_>,
        id: Pxid,
        input: UserUpdateInput,
    ) -> Result<UserUpdate> {
        UserUpdate::exec(ctx, id, input).await
    }

    async fn user_follow(&self, ctx: &Context<'_>, followee_id: Pxid) -> Result<UserFollow> {
        UserFollow::exec(ctx, followee_id).await
    }

    async fn user_unfollow(&self, ctx: &Context<'_>, followee_id: Pxid) -> Result<UserUnfollow> {
        UserUnfollow::exec(ctx, followee_id).await
    }
}
