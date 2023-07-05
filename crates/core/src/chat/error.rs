use thiserror::Error;

pub type Result<T> = std::result::Result<T, ChatError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ChatError {
    #[error("The provided PXID string instance is not valid")]
    PxidError(#[from] pxid::Error),
    #[error("User not found")]
    UserNotFound,
}
