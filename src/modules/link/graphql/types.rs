use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum LinkErrorCode {
    InvalidUrl,
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct LinkError {
    pub code: LinkErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Link {
    pub id: String,
    pub hash: String,
    pub original_url: String,
    pub expires_at: NaiveDateTime,
}
