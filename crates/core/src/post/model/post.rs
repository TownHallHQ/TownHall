use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::post::{
    error::{PostError, Result},
    repository::PostRecord,
};

pub const POST_PXID_PREFIX: &str = "post";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

pub struct NewPostDto {
    pub author_id: Pxid,
    pub parent_id: Option<Pxid>,
    pub head: bool,
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(dto: NewPostDto) -> Result<Self> {
        Ok(Self {
            id: Self::generate_id()?,
            author_id: dto.author_id,
            parent_id: dto.parent_id,
            head: dto.head,
            title: dto.title,
            content: dto.content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn generate_id() -> Result<Pxid> {
        Pxid::new(POST_PXID_PREFIX).map_err(PostError::PxidError)
    }
}

impl TryFrom<PostRecord> for Post {
    type Error = PostError;

    fn try_from(value: PostRecord) -> Result<Self> {
        let parent_id = match value.parent_id {
            Some(pid) => Some(Pxid::from_str(&pid).map_err(PostError::PxidError)?),
            None => None,
        };

        Ok(Post {
            id: Pxid::from_str(&value.id)?,
            author_id: Pxid::from_str(&value.author_id)?,
            parent_id,
            head: value.head,
            title: value.title,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
