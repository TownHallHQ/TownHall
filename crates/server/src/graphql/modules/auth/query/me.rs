use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;

use crate::graphql::modules::auth::types::{AuthError, AuthErrorCode};
use crate::graphql::modules::user::types::User;
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct Me {
    user: Option<User>,
    error: Option<AuthError>,
}

impl Me {
    pub async fn exec(ctx: &Context<'_>) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_id = ctx.data_unchecked::<Token>().user_id();

        if let Some(user) = context.services.user.find_by_id(user_id).await? {
            return Ok(Self {
                user: Some(User::from(user)),
                error: None,
            });
        }

        Ok(Self {
            user: None,
            error: Some(AuthError {
                code: AuthErrorCode::Unauthorized,
                message: String::from("Invalid credentials"),
            }),
        })
    }
}
