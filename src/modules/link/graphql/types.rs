use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use std::str::from_utf8;

use crate::modules::link::model::Link as LinkModel;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum LinkErrorCode {
    CustomHashUsed,
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
    pub original_url: String,
    pub expires_at: NaiveDateTime,
}

impl From<LinkModel> for Link {
    fn from(value: LinkModel) -> Self {
        Link {
            // FIXME: this is very expensive
            id: from_utf8(&value.id).unwrap().to_string(),
            original_url: value.original_url,
            expires_at: Default::default(),
        }
    }
}
