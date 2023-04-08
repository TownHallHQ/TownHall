use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::Result;
use crate::user::model::Email;

#[async_trait]
pub trait UserRepository: Clone {
    async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord>;
    async fn find_by_email(&self, email: Email) -> Result<Option<UserRecord>>;
    async fn find_by_id(&self, id: Pxid) -> Result<Option<UserRecord>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    /// User's Phone Number, probably can use:
    ///
    /// https://github.com/rin-nas/postgresql-patterns-library/blob/master/domains/phone.sql
    /// https://github.com/rin-nas/postgresql-patterns-library/blob/master/functions/phone_parse.sql
    pub phone: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertUserDto {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
}
