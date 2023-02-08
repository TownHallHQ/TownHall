mod link_create;

use async_graphql::{Context, Object, Result};

use link_create::LinkCreate;

use self::link_create::LinkCreateInput;

#[derive(Debug, Default)]
pub struct LinkMutationRoot;

#[Object]
impl LinkMutationRoot {
    async fn link_create(&self, ctx: &Context<'_>, input: LinkCreateInput) -> Result<LinkCreate> {
        LinkCreate::exec(ctx, input).await
    }
}
