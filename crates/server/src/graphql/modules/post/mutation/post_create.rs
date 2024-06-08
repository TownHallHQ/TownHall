use async_graphql::{Context, InputObject, Result, SimpleObject};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use townhall::auth::service::Token;
use townhall::post::service::CreatePostDto;

use crate::context::SharedContext;
use crate::graphql::modules::post::types::{Post, PostError, PostErrorCode};

#[derive(Debug, InputObject)]
pub struct PostCreateInput {
    pub title: String,
    pub content: Option<String>,
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
        let parent_id = input.parent_id;
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
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
