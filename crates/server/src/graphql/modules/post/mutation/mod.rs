use async_graphql::{Context, Object, Result};

mod post_create;

use self::post_create::{PostCreate, PostCreateInput};

#[derive(Debug, Default)]
pub struct PostMutationRoot;

#[Object]
impl PostMutationRoot {
    /// Creates a post authored by the user identified by the provided token
    async fn post_create(&self, ctx: &Context<'_>, input: PostCreateInput) -> Result<PostCreate> {
        PostCreate::exec(ctx, input).await
    }
}
