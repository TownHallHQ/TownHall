mod link_create;

use async_graphql::{Context, Object, Result};

use self::link_create::{LinkCreate, LinkCreateInput};

#[derive(Debug, Default)]
pub struct LinkMutationRoot;

#[Object]
impl LinkMutationRoot {
    async fn link_create(&self, ctx: &Context<'_>, input: LinkCreateInput) -> Result<LinkCreate> {
        LinkCreate::exec(ctx, input).await
    }
}
