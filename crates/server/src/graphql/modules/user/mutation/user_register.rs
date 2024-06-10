use std::str::FromStr;

use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use townhall::user::model::{Password, Username};
use townhall::user::service::CreateUserDto;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{User, UserError, UserErrorCode};
use crate::graphql::scalars::email::Email;

#[derive(Debug, InputObject)]
pub struct UserRegisterInput {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: Email,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserRegister {
    user: Option<User>,
    error: Option<UserError>,
}

impl UserRegister {
    pub async fn exec(ctx: &Context<'_>, input: UserRegisterInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let username = Username::from_str(&input.username)?;
        let password = Password::from_str(&input.password)?;
        let dto = CreateUserDto {
            name: input.name,
            surname: input.surname,
            username,
            email: input.email.into_inner(),
            password,
        };

        match context.services.user.register(dto).await {
            Ok(user) => Ok(Self {
                user: Some(User::from(user)),
                error: None,
            }),
            Err(err) => Ok(Self {
                user: None,
                error: Some(UserError {
                    code: UserErrorCode::Internal,
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
