use async_graphql::{Context, Result, SimpleObject};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{self, prelude::User as UserEntity};

use crate::context::SharedContext;
use crate::graphql::user::{AccessToken, UserError, UserErrorCode};

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct TokenCreate {
    token: Option<AccessToken>,
    error: Option<UserError>,
}

impl TokenCreate {
    pub async fn exec(ctx: &Context<'_>, email: String, password: String) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(user) = UserEntity::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(&context.conn())
            .await
            .unwrap()
        {
            if context
                .services
                .auth
                .validate_password(&user.hash, &password)
            {
                let access_token = context.services.auth.sign_token(user.id)?;

                return Ok(Self {
                    token: Some(AccessToken {
                        access_token: access_token.0,
                    }),
                    error: None,
                });
            }
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
