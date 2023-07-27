use async_graphql::{Context, InputObject, Result, SimpleObject};
use gabble::post::service::CreatePostDto;
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::modules::post::types::{Post, PostError, PostErrorCode};
use crate::services::auth::Token;

#[derive(Debug, InputObject)]
pub struct PostCreateInput {
    pub title: String,
    pub content: String,
    pub parent_id: Option<Pxid>,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostCreate {
    post: Option<Post>,
    error: Option<PostError>,
}

impl PostCreate {
    pub async fn exec(ctx: &Context<'_>, input: PostCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let token = ctx.data_unchecked::<Token>();

        let claims = context.services.auth.verify_token(token).unwrap();
        let parent_id = input.parent_id.map(|id| id.into_inner());
        let dto = CreatePostDto {
            author_id: claims.uid,
            parent_id,
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
                    code: PostErrorCode::Unknown,
                    message: format!("An error ocurred: {err}", err = err),
                }),
            }),
        }
    }
}
