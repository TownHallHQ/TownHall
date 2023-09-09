mod comment_create;
mod post_create;

use async_graphql::{Context, Object, Result};

use crate::graphql::guard::AuthenticationGuard;

use self::post_create::{PostCreate, PostCreateInput};

#[derive(Debug, Default)]
pub struct PostMutationRoot;

#[Object]
impl PostMutationRoot {
    /// Creates a post authored by the user identified by the provided token
    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn post_create(&self, ctx: &Context<'_>, input: PostCreateInput) -> Result<PostCreate> {
        PostCreate::exec(ctx, input).await
    }

    /// Creates a comment authored by the user identified by the provided token
    #[graphql(guard = "AuthenticationGuard::new()")]
    async fn comment_create(
        &self,
        ctx: &Context<'_>,
        input: comment_create::CommentCreateInput,
    ) -> Result<comment_create::CommentCreate> {
        comment_create::CommentCreate::exec(ctx, input).await
    }
}
