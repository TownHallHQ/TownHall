use async_graphql::{Enum, SimpleObject, ID};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::modules::link::model::Link as LinkModel;

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

impl From<LinkModel> for Link {
    fn from(value: LinkModel) -> Self {
        Link {
            id: ID(value.id),
            hash: value.hash,
            original_url: value.original_url,
            expires_at: Default::default(),
        }
    }
}
