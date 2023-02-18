use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::modules::user::graphql::{AccessToken, UserError, UserErrorCode};
use crate::shared::repository::Repository;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    token: Option<AccessToken>,
    error: Option<UserError>,
}

impl TokenCreate {
    pub async fn exec(ctx: &Context<'_>, email: String, password: String) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let Ok(maybe_user) = context
            .repositories
            .user
            .find_by_key(email.as_bytes()) else {
                tracing::error!(%email, "Failed to retrieve user from repository");

                return Ok(Self {
                    token: None,
                    error: Some(UserError {
                        code: UserErrorCode::Unknown,
                        message: String::from("An error ocurred")
                    }),
                });
            };

        let Some(user) = maybe_user else {
                tracing::error!(%email, "User with email wasn't found");

                return Ok(Self {
                    token: None,
                    error: Some(UserError {
                        code: UserErrorCode::Unauthorized,
                        message: String::from("Invalid Credentials")
                    }),
                });
            };

        if context
            .services
            .auth
            .validate_password(&user.password_hash, &password)
        {
            let Ok(access_token) = context.services.auth.sign_token(email) else {
                tracing::error!("Failed to sign token");

                return Ok(Self {
                    token: None,
                    error: Some(UserError {
                        code: UserErrorCode::Unknown,
                        message: String::from("An error ocurred")
                    }),
                });
            };

            return Ok(Self {
                token: Some(AccessToken {
                    access_token: access_token.0,
                }),
                error: None,
            });
        }

        tracing::warn!("Invalid credentials provided");

        Ok(Self {
            token: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid credentials"),
            }),
        })
    }
}
