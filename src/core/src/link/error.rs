use thiserror::Error;

pub type Result<T> = std::result::Result<T, LinkError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum LinkError {
    #[error("Failed to parse a UID instance from \"{0}\"")]
    UlidParseError(String),
    #[error("An error ocurred processing Pxid value")]
    PxidError(pxid::Error),
    #[error("Failed to parse URL from string instance \"{0}\"")]
    UrlParseError(String),
    #[error("Database Error")]
    DatabaseError,
}

impl From<pxid::Error> for LinkError {
    fn from(value: pxid::Error) -> Self {
        LinkError::PxidError(value)
    }
}
