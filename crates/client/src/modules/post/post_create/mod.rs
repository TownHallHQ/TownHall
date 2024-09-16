use anyhow::{anyhow, Result};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use pxid::Pxid;

use post_create::{
    PostCreateInput, PostCreatePostCreateError, PostCreatePostCreatePost, Variables,
};

use crate::{DateTime, GRAPHQL_PATH};

use super::PostClient;

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Clone,Debug,Deserialize",
    schema_path = "schema.json",
    query_path = "src/modules/post/post_create/PostCreate.gql"
)]
pub struct PostCreate {
    pub post: Option<PostCreatePostCreatePost>,
    pub error: Option<PostCreatePostCreateError>,
}

pub async fn posts(post_client: &PostClient, input: PostCreateInput) -> Result<PostCreate> {
    let url = post_client.domain.join(GRAPHQL_PATH)?;
    let variables = Variables { input };

    let res = post_graphql::<PostCreate, _>(&post_client.client, url, variables)
        .await
        .map_err(|err| anyhow!("Failed to create post. {err}"))?;
    let data = res.data.unwrap().post_create;

    Ok(PostCreate {
        post: data.post,
        error: data.error,
    })
}
