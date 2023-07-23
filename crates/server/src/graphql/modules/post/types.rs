use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

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
