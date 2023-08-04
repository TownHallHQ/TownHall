use async_graphql::{Context, Result, SimpleObject};
use gabble::shared::pagination::Pagination;
use gabble::user::repository::UserFilter;
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;

use crate::graphql::modules::user::types::{User, UserError, UserErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct Me {
    user: Option<User>,
    error: Option<UserError>,
}

impl Me {
    pub async fn exec(ctx: &Context<'_>) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();
            let records = context
                .services
                .user
                .list(
                    Some(Pagination::first()),
                    Some(UserFilter {
                        id: Some(claims.uid),
                        ..Default::default()
                    }),
                )
                .await?;

            if records.len() != 1 {
                return Ok(Me {
                    user: None,
                    error: Some(UserError {
                        code: UserErrorCode::Unauthorized,
                        message: String::from("Invalid token provided"),
                    }),
                });
            }
            let user = records.first().unwrap().to_owned();

            return Ok(Me {
                user: Some(User::from(user)),
                error: None,
            });
        }

        Ok(Self {
            user: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }
}
