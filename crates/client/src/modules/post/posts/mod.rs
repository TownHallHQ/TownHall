use anyhow::{anyhow, bail, Result};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use posts::{PostsPostsEdges, PostsPostsPageInfo, Variables};
use pxid::Pxid;
use serde::Serialize;

use crate::{DateTime, GRAPHQL_PATH};

use super::PostClient;

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Clone,Debug,Deserialize,Serializable",
    schema_path = "schema.json",
    query_path = "src/modules/post/posts/Posts.gql"
)]
pub struct Posts {
    pub edges: Vec<PostsPostsEdges>,
    pub total_count: i64,
    pub page_info: PostsPostsPageInfo,
}

pub async fn posts(
    public_client: &PostClient,
    after: Option<Pxid>,
    before: Option<Pxid>,
    first: Option<i64>,
    last: Option<i64>,
) -> Result<Posts> {
    let url = public_client.domain.join(GRAPHQL_PATH)?;

    let res = post_graphql::<Posts, _>(
        &public_client.client,
        url,
        Variables {
            after,
            before,
            first,
            last,
        },
    )
    .await
    .map_err(|err| anyhow!("Failed to make request. {err}"))?;

    if let Some(ref data) = res.data {
        return Ok(Posts {
            edges: data.posts.edges.to_owned(),
            total_count: data.posts.total_count,
            page_info: data.posts.page_info.to_owned(),
        });
    }

    bail!("Failed to get posts. err = {:#?}", res.errors.unwrap())
}
