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

        match context
            .services
            .user
            .verify_credentials(&email, &password)
            .await
        {
            Ok(credentials_ok) => {
                if credentials_ok {
                    if let Some(user) = context.services.user.find_by_email(&email).await? {
                        let token = context.services.auth.sign_token(user.id)?;

                        return Ok(Self {
                            token: Some(AccessToken {
                                access_token: token.token_string(),
                            }),
                            error: None,
                        });
                    }
                }

                Ok(Self {
                    token: None,
                    error: Some(AuthError {
                        code: AuthErrorCode::Unauthorized,
                        message: String::from("Invalid credentials"),
                    }),
                })
            }
            Err(err) => {
                tracing::error!(%err, "Failed to verify credentials");

                Ok(Self {
                    token: None,
                    error: Some(AuthError {
                        code: AuthErrorCode::Internal,
                        message: String::from("An error ocurred"),
                    }),
                })
            }
        }
    }
}
