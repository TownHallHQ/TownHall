use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum AuthErrorCode {
    EmailParseError,
    EmailTaken,
    Unauthorized,
    Internal,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AuthError {
    pub code: AuthErrorCode,
    pub message: String,
}
