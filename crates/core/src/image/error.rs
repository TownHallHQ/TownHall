use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImageError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ImageError {
    #[error("The provided PXID string instance is not valid")]
    PxidError(#[from] pxid::Error),
    #[error("Image not found")]
    NotFound,
    /// The Image provided is too big for the current _use_case_, the first
    /// param from the tuple represents the provided image size, while the
    /// second represents the maximum allowed
    #[error("Image file is too big, provided size is {0} and maximum is {1}")]
    ImageFileTooBig(usize, usize),
    #[error("Empty image bytes found")]
    EmptyBytes,
    #[error("Repository layer error")]
    RepositoryError,
    #[error("An error ocurred in the provider layer. {0}")]
    ProviderError(String),
}
