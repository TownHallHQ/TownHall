use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::chat::error::{ChatError, Result};
use crate::chat::repository::ChatRecord;

pub const CHAT_PXID_PREFIX: &str = "chat";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Chat {
    pub id: Pxid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<ChatRecord> for Chat {
    type Error = ChatError;

    fn try_from(value: ChatRecord) -> Result<Self> {
        Ok(Chat {
            id: Pxid::from_str(&value.id)?,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
