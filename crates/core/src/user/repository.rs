use async_trait::async_trait;
use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::user::error::Result;
use crate::user::model::Email;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFilter {
    pub id: Option<Pxid>,
    pub email: Option<Email>,
}

#[async_trait]
pub trait UserRepository: Clone {
    async fn insert(&self, dto: InsertUserDto) -> Result<UserRecord>;
    async fn find(&self, filter: Option<UserFilter>) -> Result<Vec<UserRecord>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserRecord {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
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
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
