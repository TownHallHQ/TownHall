use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    context::SharedContext,
    modules::user::graphql::{AccessToken, UserError, UserErrorCode},
};

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    token: Option<AccessToken>,
    error: Option<UserError>,
}

impl TokenCreate {
    pub async fn exec(ctx: &Context<'_>, email: String, password: String) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        let user = context.services.user.find_by_email(email).unwrap();

        if context
            .services
            .auth
            .validate_password(&user.hash, &password)
        {
            let access_token = context.services.auth.sign_token(user.id).unwrap();

            return Ok(Self {
                token: Some(AccessToken {
                    access_token: access_token.0,
                }),
                error: None,
            });
        }

        Ok(Self {
            token: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid credentials"),
            }),
        })
    }
}
