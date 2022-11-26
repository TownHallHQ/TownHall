mod link_list;

use async_graphql::{Context, Object, Result};

use self::link_list::LinkList;

#[derive(Debug, Default)]
pub struct LinkQueryRoot;

#[Object]
impl LinkQueryRoot {
    async fn link_list(&self, ctx: &Context<'_>) -> Result<LinkList> {
        LinkList::exec(ctx).await
    }
}
