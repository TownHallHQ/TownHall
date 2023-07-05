use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::chat::error::Result;

#[async_trait]
pub trait ChatRepository: Clone {
    async fn insert(&self, dto: InsertChatDto) -> Result<ChatRecord>;
    async fn find_all(&self, whoami: Pxid) -> Result<Vec<ChatRecord>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChatRecord {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct InsertChatDto {
    pub whoami: Pxid,
    pub peer_id: Pxid,
}
