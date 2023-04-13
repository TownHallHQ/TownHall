use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::link::error::Result;
use crate::link::model::fingerprint::Fingerprint;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LinkRecord {
    pub id: String,
    pub fingerprint: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertLinkDto {
    pub user_id: String,
    pub original_url: String,
    pub fingerprint: Option<String>,
}

pub struct LinkFilter {
    pub id: Option<Pxid>,
    pub fingerprint: Option<Fingerprint>,
}

#[async_trait]
pub trait LinkRepository: Clone {
    async fn insert(&self, dto: InsertLinkDto) -> Result<LinkRecord>;
    async fn find(&self, filter: Option<LinkFilter>) -> Result<Vec<LinkRecord>>;
}
