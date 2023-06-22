use thiserror::Error;

use super::model::EmailError;
use super::model::PasswordError;
use super::model::UsernameError;

pub type Result<T> = std::result::Result<T, UserError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum UserError {
    #[error("The username is invalid")]
    UsernameError(UsernameError),
    #[error("An error ocurred processing email value")]
    EmailError(EmailError),
    #[error("The email {0} is already taken")]
    EmailTakenError(String),
    #[error("An error ocurred processing password value")]
    PasswordError(PasswordError),
    #[error("The provided PXID string instance is not valid")]
    PxidError(pxid::Error),
    #[error("Database layer error")]
    DatabaseError,
    #[error("User not found")]
    UserNotFound,
}

impl From<EmailError> for UserError {
    fn from(value: EmailError) -> Self {
        UserError::EmailError(value)
    }
}

impl From<UsernameError> for UserError {
    fn from(value: UsernameError) -> Self {
        UserError::UsernameError(value)
    }
}

impl From<PasswordError> for UserError {
    fn from(value: PasswordError) -> Self {
        UserError::PasswordError(value)
    }
}

impl From<pxid::Error> for UserError {
    fn from(value: pxid::Error) -> Self {
        UserError::PxidError(value)
    }
}
