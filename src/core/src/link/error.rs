use thiserror::Error;

pub type Result<T> = std::result::Result<T, LinkError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum LinkError {
    #[error("Invalid URL Provided")]
    ParseUrlError(#[from] url::ParseError),
    #[error("The provided PXID string instance is not valid")]
    PxidError(#[from] pxid::Error),
    #[error("Repository layer error")]
    RepositoryError,
    #[error("Link not found")]
    LinkNotFound,
}
