use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use townhall::user::repository::user::UpdateUserDto;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{User, UserError, UserErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, InputObject)]
pub struct UserUpdateInput {
    pub name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserUpdate {
    user: Option<User>,
    error: Option<UserError>,
}

impl UserUpdate {
    pub async fn exec(ctx: &Context<'_>, input: UserUpdateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_id = ctx.data_unchecked::<Token>().user_id();
        let dto = UpdateUserDto {
            name: input.name,
            surname: input.surname,
        };

        match context.services.user.update(user_id, dto).await {
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
