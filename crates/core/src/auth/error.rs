use thiserror::Error;

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("Failed to sign token")]
    SignTokenError,
    #[error("Failed to encode JWT")]
    JwtEncodeDecodeError(#[from] jsonwebtoken::errors::Error),
}
