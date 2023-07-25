mod posts;

use async_graphql::{Context, Object, Result};
use pxid::graphql::Pxid;

use self::posts::{Posts, PostsConnection};

#[derive(Debug, Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn posts(
        &self,
        ctx: &Context<'_>,
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<PostsConnection> {
        Posts::exec(
            ctx,
            after.map(|p| p.to_string()),
            before.map(|p| p.to_string()),
            first,
            last,
        )
        .await
    }
}
