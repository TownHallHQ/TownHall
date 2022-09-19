use axum::body::Full;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use serde::Serialize;

pub mod create_link;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    pub fn new(message: &str, status: StatusCode) -> Self {
        Self {
            message: message.to_string(),
            status: status.as_u16(),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError {
            message: error.to_string(),
            status: 500,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(self.status)
            .body(Full::from(self.message))
            .unwrap()
            .into_response()
    }
}
