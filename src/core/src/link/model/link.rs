use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::link::error::{LinkError, Result};

use super::Url;

pub const LINK_PXID_PREFIX: &str = "link";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Link {
    pub id: Pxid,
    pub hash: String,
    pub url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Link {
    pub fn new_id() -> Result<Pxid> {
        Pxid::new(LINK_PXID_PREFIX).map_err(LinkError::PxidError)
    }
}
