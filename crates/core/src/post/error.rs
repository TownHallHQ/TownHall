use thiserror::Error;

pub type Result<T> = std::result::Result<T, PostError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PostError {
    #[error("The provided PXID string instance is not valid")]
    PxidError(pxid::Error),
    #[error("Database layer error")]
    DatabaseError,
}

impl From<pxid::Error> for PostError {
    fn from(value: pxid::Error) -> Self {
        PostError::PxidError(value)
    }
}
