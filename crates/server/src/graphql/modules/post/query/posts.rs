use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Result};

use crate::context::SharedContext;

use crate::graphql::modules::post::types::Post;

pub type PostsConnection = Connection<String, Post, EmptyFields, EmptyFields>;

pub struct Posts;

impl Posts {
    pub async fn exec(
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<PostsConnection> {
        let context = ctx.data_unchecked::<SharedContext>();

        query(
            after,
            before,
            first,
            last,
            |_after, _before, _first, _last| async move {
                let posts = context.services.post.list().await?;
                let mut connection = Connection::new(false, false);

                connection.edges.extend(posts.into_iter().filter_map(|p| {
                    match Post::try_from(p) {
                        Ok(p) => Some(Edge::new(p.id.to_string(), p)),
                        Err(err) => {
                            tracing::error!(%err, "Failed to create post instance from result");
                            None
                        }
                    }
                }));

                Ok::<PostsConnection, async_graphql::Error>(connection)
            },
        )
        .await
    }
}
