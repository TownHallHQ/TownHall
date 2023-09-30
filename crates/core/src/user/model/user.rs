use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::{Result, UserError};
use crate::user::repository::user::UserRecord;

use super::email::Email;
use super::password::Password;
use super::username::Username;

pub const USER_PXID_PREFIX: &str = "user";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: Pxid,
    pub name: String,
    pub surname: String,
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub avatar_id: Option<Pxid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct NewUserDto {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(dto: NewUserDto) -> Result<Self> {
        let email = Email::from_str(&dto.email)?;
        let username = Username::from_str(&dto.username)?;
        let password = Password::from_str(&dto.password)?;

        Ok(Self {
            id: Self::generate_id()?,
            name: dto.name,
            surname: dto.surname,
            username,
            email,
            password,
            avatar_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub fn generate_id() -> Result<Pxid> {
        Pxid::new(USER_PXID_PREFIX).map_err(UserError::PxidError)
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
            password: Password(value.password_hash.into()),
            avatar_id: value.avatar_id.map(|id| Pxid::from_str(&id)).transpose()?,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        })
    }
}
