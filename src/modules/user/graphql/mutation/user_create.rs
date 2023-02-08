use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    context::SharedContext,
    modules::{
        auth::graphql::User,
        user::{graphql::UserError, repository::CreateUserDto},
    },
};

#[derive(Debug, Default, InputObject)]
pub struct UserCreateInput {
    pub name: String,
    pub last_name: String,
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
        let hash = context
            .services
            .auth
            .hash_password(&input.password)
            .unwrap();

        let user = CreateUserDto {
            name: input.name,
            last_name: input.last_name,
            email: input.email,
            hash,
        };

        let result = context.services.user.create(user);
        let response = User::from(result);

        Ok(Self {
            user: Some(response),
            error: None,
        })
    }
}
