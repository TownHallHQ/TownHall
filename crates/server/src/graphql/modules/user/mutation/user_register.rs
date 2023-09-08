use std::str::FromStr;

use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use playa::user::model::{Email, Password, Username};
use playa::user::service::CreateUserDto;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{User, UserError, UserErrorCode};

#[derive(Debug, Default, InputObject)]
pub struct UserRegisterInput {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
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
        let email = Email::from_str(&input.email)?;
        let username = Username::from_str(&input.username)?;
        let password = Password::from_str(&input.password)?;
        let dto = CreateUserDto {
            name: input.name,
            surname: input.surname,
            username,
            email,
            password,
        };

        match context.services.user.create(dto).await {
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
