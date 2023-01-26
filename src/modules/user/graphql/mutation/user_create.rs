use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::{context::SharedContext, modules::user::service::CreateUserDto};

#[derive(Debug, Default, InputObject)]
pub struct UserCreateInput {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserCreate {
    user: String,
    // user: Option<User>,
    // error: Option<UserError>,
}

impl UserCreate {
    pub async fn exec(ctx: &Context<'_>, input: UserCreateInput) -> Result<String> {
        let context = ctx.data_unchecked::<SharedContext>();
        // auth - hash_password

        let user = CreateUserDto {
            name: input.name,
            last_name: input.last_name,
            email: input.email,
        };

        Ok(context.services.user.create(user))
    }
}
