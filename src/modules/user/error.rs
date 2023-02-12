use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("The provided email {0} is already taken")]
    EmailTaken(String),
}
