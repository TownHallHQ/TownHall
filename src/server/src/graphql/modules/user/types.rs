use async_graphql::{Enum, SimpleObject, ID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    InvalidEmail,
    EmailTaken,
    Unauthorized,
    Internal,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    pub code: UserErrorCode,
    pub message: String,
}

/// A Platform's User
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<gabble::user::model::User> for User {
    fn from(value: gabble::user::model::User) -> Self {
        User {
            id: ID(value.id.to_string()),
            name: value.name,
            surname: value.surname,
            email: value.email.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
