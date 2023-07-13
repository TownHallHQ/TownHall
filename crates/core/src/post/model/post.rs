use chrono::{DateTime, Utc};
use pxid::Pxid;

use crate::post::error::{PostError, Result};

pub const POST_PXID_PREFIX: &str = "post";

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
