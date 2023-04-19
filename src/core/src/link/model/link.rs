use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use url::Url;

use serde::{Deserialize, Serialize};

use crate::link::error::LinkError;
use crate::link::repository::LinkRecord;

use super::ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Link {
    pub id: Pxid,
    pub ulid: Ulid,
    pub original_url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Link {
    pub fn generate_id() -> Pxid {
        Pxid::new_unchecked("link")
    }
}

impl TryFrom<LinkRecord> for Link {
    type Error = LinkError;

    fn try_from(value: LinkRecord) -> Result<Self, Self::Error> {
        let id = Pxid::from_str(&value.id)?;
        let ulid = Ulid::from_str(&value.ulid)?;
        let original_url = Url::from_str(&value.original_url)
            .map_err(|_err| LinkError::UrlParseError(value.original_url))?;

        Ok(Self {
            id,
            ulid,
            original_url,
            created_at: value.created_at,
            deleted_at: value.deleted_at,
            updated_at: value.updated_at,
        })
    }
}
