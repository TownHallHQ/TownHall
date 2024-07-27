use std::str::FromStr;
use std::sync::Arc;

use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use http_auth_basic::Credentials;
use tracing::instrument;

use townhall::user::model::Email;

use crate::context::Context;

#[instrument(skip(ctx))]
pub async fn handler(Extension(ctx): Extension<Arc<Context>>, req: Request) -> Response {
    let auth_header = req.headers().get(AUTHORIZATION);

    if let Some(auth_header) = auth_header {
        let Ok(header_value) = auth_header.to_str() else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        let Ok(credentials) = Credentials::from_str(header_value) else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        let Ok(email) = Email::from_str(&credentials.user_id) else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        match ctx
            .services
            .user
            .verify_credentials(&email, &credentials.password)
            .await
        {
            Ok(credentials_ok) => {
                if credentials_ok {
                    if let Some(user) = ctx.services.user.find_by_email(&email).await.unwrap() {
                        let token = ctx.services.auth.sign_token(user.id).unwrap();

                        return token.to_string().into_response();
                    }
                }

                return StatusCode::BAD_REQUEST.into_response();
            }
            Err(err) => {
                tracing::error!(%err, "Failed to verify credentials");

                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }

    StatusCode::UNAUTHORIZED.into_response()
}
