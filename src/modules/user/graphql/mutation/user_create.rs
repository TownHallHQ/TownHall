use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{
    context::SharedContext,
    modules::user::{graphql::UserError, service::CreateUserDto},
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
    user: Option<String>,
    error: Option<UserError>,
}

impl UserCreate {
    pub async fn exec(ctx: &Context<'_>, input: UserCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        // auth - hash_password

        let user = CreateUserDto {
            name: input.name,
            last_name: input.last_name,
            email: input.email,
        };

        Ok(Self {
            user: Some(context.services.user.create(user)),
            error: None,
        })
    }
}
