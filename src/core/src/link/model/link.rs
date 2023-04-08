use chrono::{DateTime, Utc};
use pxid::Pxid;
use url::Url;

use super::fingerprint::Fingerprint;

pub struct Link {
    pub id: Pxid,
    pub fingerprint: Fingerprint,
    pub original_url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
