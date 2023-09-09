use crate::context::SharedContext;
use crate::graphql::modules::post::types::{Post, PostError};
use crate::services::auth::Token;
use async_graphql::{Context, InputObject, Result, SimpleObject};
use playa::post::service::CreatePostDto;
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};
#[derive(Debug, InputObject)]
pub struct CommentCreateInput {
    pub content: String,
    pub parent_id: Pxid,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct CommentCreate {
    comment: Option<Post>,
    error: Option<PostError>,
}

impl CommentCreate {
    pub async fn exec(ctx: &Context<'_>, input: CommentCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let token = ctx.data_unchecked::<Token>();
        let claims = context.services.auth.verify_token(token)?;
        let dto = CreatePostDto {
            author_id: claims.uid,
            parent_id: Some(input.parent_id.into_inner()),
            title: "".to_string(),
            content: Some(input.content),
        };

        match context.services.post.create(dto).await {
            Ok(post) => Ok(Self {
                comment: Some(Post::from(post)),
                error: None,
            }),
            Err(err) => Ok(Self {
                comment: None,
                error: Some(PostError {
                    code: crate::graphql::modules::post::types::PostErrorCode::Unknown,
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
