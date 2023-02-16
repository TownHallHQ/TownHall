use std::str::from_utf8;

use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::modules::user::graphql::{User, UserError, UserErrorCode};
use crate::modules::user::repository::CreateUserDto;
use crate::shared::repository::Repository;

#[derive(Debug, Default, InputObject)]
pub struct UserCreateInput {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserCreate {
    user: Option<User>,
    error: Option<UserError>,
}

impl UserCreate {
    pub async fn exec(ctx: &Context<'_>, input: UserCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let password_hash = context
            .services
            .auth
            .hash_password(&input.password)
            .unwrap();
        let email_bytes = input.email.as_bytes().to_vec();
        let create_user_dto = CreateUserDto {
            name: input.name,
            surname: input.surname,
            email: input.email,
            password_hash,
        };
        let maybe_user = context.repositories.user.find_by_key(&email_bytes).unwrap();

        if maybe_user.is_none() {
            let user = context
                .repositories
                .user
                .create_with_key(email_bytes, create_user_dto)
                .unwrap();

            return Ok(Self {
                user: Some(User::from(user)),
                error: None,
            });
        }

        Ok(Self {
            user: None,
            error: Some(UserError {
                code: UserErrorCode::EmailTaken,
                message: format!(
                    "Email {email} is already taken",
                    email = from_utf8(&email_bytes).unwrap()
                ),
            }),
        })
    }
}
