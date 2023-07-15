use async_graphql::{Context, Result};
use gabble::post::service::CreatePostDto;
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::{
    context::SharedContext,
    graphql::modules::post::types::{Post, PostError, PostErrorCode},
};

#[derive(Debug, Default)]
pub struct PostCreateInput {
    pub author_id: String,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PostCreate {
    post: Option<Post>,
    error: Option<PostError>,
}

impl PostCreate {
    pub async fn exec(ctx: &Context<'_>, input: PostCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let dto = CreatePostDto {
            author_id: input.author_id,
            parent_id: input.parent_id,
            head: input.head,
            title: input.title,
            content: input.content,
        };

        match context.services.post.create(dto).await {
            Ok(post) => Ok(Self {
                post: Some(Post::from(post)),
                error: None,
            }),
            Err(err) => Ok(Self {
                post: None,
                error: Some(PostError {
                    code: PostErrorCode::Internal,
                    message: format!("An error ocurred: {err}", err = err),
                }),
            }),
        }
    }
}
