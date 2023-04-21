use std::str::FromStr;

use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use quicklink::user::model::Email;
use quicklink::user::repository::UserFilter;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{AccessToken, UserError, UserErrorCode};

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    token: Option<AccessToken>,
    error: Option<UserError>,
}

impl TokenCreate {
    pub async fn exec(ctx: &Context<'_>, email: String, password: String) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let Ok(email) = Email::from_str(&email) else {
            tracing::warn!(%email, "Invalid email provided");

            return Ok(Self {
                token: None,
                error: Some(UserError {
                    code: UserErrorCode::EmailTaken,
                    message: String::from("Invalid email provided")
                }),
            });
          };

        let Ok(records) = context
            .services
            .user
            .find(Some(UserFilter {
              email: Some(email),
              ..Default::default()
            })).await else {
                tracing::error!("Failed to retrieve user from repository");

                return Ok(Self {
                    token: None,
                    error: Some(UserError {
                        code: UserErrorCode::Internal,
                        message: String::from("An error ocurred")
                    }),
                });
            };

        if records.len() != 1 {
            tracing::error!("More than 1 record found");

            return Ok(Self {
                token: None,
                error: Some(UserError {
                    code: UserErrorCode::Unauthorized,
                    message: String::from("Invalid credentials"),
                }),
            });
        }

        let user = records.get(0).unwrap();

        if context
            .services
            .auth
            .validate_password(&user.password.to_string(), &password)
        {
            let Ok(access_token) = context.services.auth.sign_token(user.id) else {
                tracing::error!("Failed to sign token");

                return Ok(Self {
                    token: None,
                    error: Some(UserError {
                        code: UserErrorCode::Internal,
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
