use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::{Result, UserError};
use crate::user::repository::UserRecord;

use super::email::Email;
use super::password::Password;

pub const USER_PXID_PREFIX: &str = "user";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: Pxid,
    pub name: String,
    pub surname: String,
    pub email: Email,
    pub password: Password,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct NewUserDto {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(dto: NewUserDto) -> Result<Self> {
        let email = Email::from_str(&dto.email)?;
        let password = Password::from_str(&dto.password)?;

        Ok(Self {
            id: Self::generate_id()?,
            name: dto.name,
            surname: dto.surname,
            email,
            password,
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
            email: Email(value.email.into()),
            password: Password(value.password_hash.into()),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
        })
    }
}
