pub mod create_link;
pub mod graphql;
pub mod redirect;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Clone, Debug, Serialize)]
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
        let mut response = Json(self.clone()).into_response();

        // Here we `unwrap` explicitly because the `status` is validated
        // on `struct` initialization when calling (`ApiError::new`).
        *response.status_mut() = StatusCode::from_u16(self.status).unwrap();
        response
    }
}
