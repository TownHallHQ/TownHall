use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use super::email::Email;
use super::username::Username;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: Pxid,
    pub name: String,
    pub surname: String,
    pub username: Username,
    pub email: Email,
    pub avatar_id: Option<Pxid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
