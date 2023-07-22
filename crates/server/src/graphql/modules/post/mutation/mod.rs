use async_graphql::{Context, Object, Result};

mod post_create;

use self::post_create::PostCreate;

#[derive(Debug, Default)]
pub struct PostMutationRoot;

#[Object]
impl PostMutationRoot {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: post_create::PostCreateInput,
    ) -> Result<PostCreate> {
        PostCreate::exec(ctx, input).await
    }
}
