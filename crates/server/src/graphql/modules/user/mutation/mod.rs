mod user_avatar_update;
mod user_follow;
mod user_register;
mod user_unfollow;
mod user_update;

use async_graphql::{Context, Object, Result, Upload};
use pxid::Pxid;

use crate::graphql::guard::AuthenticationGuard;

use self::user_avatar_update::UserAvatarUpdate;
use self::user_follow::UserFollow;
use self::user_register::{UserRegister, UserRegisterInput};
use self::user_unfollow::UserUnfollow;
use self::user_update::{UserUpdate, UserUpdateInput};

#[derive(Debug, Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        input: UserRegisterInput,
    ) -> Result<UserRegister> {
        UserRegister::exec(ctx, input).await
    }

    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn user_update(&self, ctx: &Context<'_>, input: UserUpdateInput) -> Result<UserUpdate> {
        UserUpdate::exec(ctx, input).await
    }

    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn user_follow(&self, ctx: &Context<'_>, followee_id: Pxid) -> Result<UserFollow> {
        UserFollow::exec(ctx, followee_id).await
    }

    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn user_unfollow(&self, ctx: &Context<'_>, followee_id: Pxid) -> Result<UserUnfollow> {
        UserUnfollow::exec(ctx, followee_id).await
    }

    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn user_avatar_update(
        &self,
        ctx: &Context<'_>,
        file: Upload,
    ) -> Result<UserAvatarUpdate> {
        UserAvatarUpdate::exec(ctx, file).await
    }
}
