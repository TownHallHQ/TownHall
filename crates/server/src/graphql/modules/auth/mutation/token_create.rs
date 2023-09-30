use async_graphql::{Context, Result, SimpleObject};

use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::graphql::modules::auth::types::{AccessToken, AuthError, AuthErrorCode};
use crate::graphql::scalars::email::Email;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    token: Option<AccessToken>,
    error: Option<AuthError>,
}

impl TokenCreate {
    pub async fn exec(ctx: &Context<'_>, email: Email, password: String) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let Some(user) = context.services.user.find_by_email(&email).await? else {
            return Ok(Self {
                token: None,
                error: Some(AuthError {
                    code: AuthErrorCode::Unauthorized,
                    message: String::from("Invalid credentials"),
                }),
            });
        };

        if context
            .services
            .auth
            .validate_password(&user.password.to_string(), &password)
        {
            let Ok(access_token) = context.services.auth.sign_token(user.id) else {
                tracing::error!("Failed to sign token");

                return Ok(Self {
                    token: None,
                    error: Some(AuthError {
                        code: AuthErrorCode::Internal,
                        message: String::from("An error ocurred"),
                    }),
                });
            };

            return Ok(Self {
                token: Some(AccessToken {
                    access_token: access_token.raw,
                }),
                error: None,
            });
        }

        tracing::warn!("Invalid credentials provided");
        Ok(Self {
            token: None,
            error: Some(AuthError {
                code: AuthErrorCode::Unauthorized,
                message: String::from("Invalid credentials"),
            }),
        })
    }
}
