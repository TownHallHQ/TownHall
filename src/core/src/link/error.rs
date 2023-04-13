use thiserror::Error;

pub type Result<T> = std::result::Result<T, LinkError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum LinkError {
    #[error("Failed to parse a Fingerprint instance from \"{0}\"")]
    FingerprintParseError(String),
    #[error("An error ocurred processing Pxid value")]
    PxidError(pxid::Error),
    #[error("Database Error")]
    DatabaseError,
}

impl From<pxid::Error> for LinkError {
    fn from(value: pxid::Error) -> Self {
        LinkError::PxidError(value)
    }
}
