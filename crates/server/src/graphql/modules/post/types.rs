use async_graphql::{Enum, SimpleObject, ID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum PostErrorCode {
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct PostError {
    pub code: PostErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: ID,
    pub author_id: ID,
    pub parent_id: Option<ID>,
    pub head: bool,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<gabble::post::model::Post> for Post {
    fn from(value: gabble::post::model::Post) -> Self {
        Post {
            id: ID(value.id.to_string()),
            author_id: ID(value.author_id.to_string()),
            parent_id: Some(value.parent_id.map(|pxid| ID(pxid.to_string())).unwrap()),
            head: value.head,
            title: value.title,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
