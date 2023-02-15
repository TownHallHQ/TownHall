use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    EmailTaken,
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    pub code: UserErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}
