use chrono::{DateTime, Utc};
use pxid::Pxid;
use url::Url;

use serde::{Deserialize, Serialize};

use super::fingerprint::Fingerprint;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Link {
    pub id: Pxid,
    pub fingerprint: Fingerprint,
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
