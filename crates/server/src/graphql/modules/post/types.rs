use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};
use gabble::{shared::pagination::Pagination, user::repository::UserFilter};
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::modules::user::types::User;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum PostErrorCode {
    InvalidParentId,
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostError {
    pub code: PostErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Post {
    pub id: Pxid,
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Post {
    pub async fn author(&self, ctx: &Context<'_>) -> User {
        // Safely uses `unwrap` given that a Post cannot exist without an author
        let context = ctx.data_unchecked::<SharedContext>();
        let qs = context
            .services
            .user
            .list(
                Some(Pagination::first()),
                Some(UserFilter {
                    id: Some(self.author_id.into_inner()),
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        let user = qs.first().unwrap().to_owned();

        User::try_from(user).unwrap()
    }
}

impl From<gabble::post::model::Post> for Post {
    fn from(value: gabble::post::model::Post) -> Self {
        Post {
            id: value.id.into(),
            author_id: value.author_id.into(),
            parent_id: value.parent_id.map(|id| id.into()),
            head: value.head,
            title: value.title,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
