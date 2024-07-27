mod email;
mod username;

pub use email::*;
pub use username::*;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

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
