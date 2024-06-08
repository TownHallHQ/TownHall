use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::{Result, UserError};
use crate::user::repository::user::UserRecord;
use crate::PXID_GENERATOR;

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

impl User {
    pub fn pxid() -> Result<Pxid> {
        const USER_PXID_PREFIX: &str = "user";
        Ok(PXID_GENERATOR.new_id(USER_PXID_PREFIX)?)
    }
}

impl TryFrom<UserRecord> for User {
    type Error = UserError;

    fn try_from(value: UserRecord) -> Result<Self> {
        Ok(User {
            id: Pxid::from_str(&value.id)?,
            name: value.name,
            surname: value.surname,
            username: Username(value.username.into()),
            email: Email(value.email.into()),
            avatar_id: value.avatar_id.map(|id| Pxid::from_str(&id)).transpose()?,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        })
    }
}
