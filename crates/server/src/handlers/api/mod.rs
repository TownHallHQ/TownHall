pub mod v1;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Router;
use serde::Serialize;

pub type Result = std::result::Result<Box<dyn IntoResponse>, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    status_code: u16,
}

impl ApiError {
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status_code: status_code.as_u16(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::BAD_REQUEST)
    }

    pub fn unprocessable_entity(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::UNPROCESSABLE_ENTITY)
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let Ok(response) = Response::builder()
            .status(self.status_code)
            .body(self.message.clone())
        {
            return response.into_response();
        }

        ApiError::new("Server layer error", StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

pub fn api() -> Router {
    Router::new().nest("/api/v1", v1::v1())
}
