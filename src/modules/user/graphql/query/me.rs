use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::modules::auth::{graphql::User, service::Token};
use crate::modules::user::graphql::{UserError, UserErrorCode};
use crate::shared::repository::Repository;

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
            let user = context
                .repositories
                .user
                .find_by_key(claims.uid.as_bytes())
                .unwrap();
            let result = User::from(user.unwrap());

            return Ok(Me {
                user: Some(result),
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
