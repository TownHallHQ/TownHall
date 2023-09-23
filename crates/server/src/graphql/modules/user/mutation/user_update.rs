use async_graphql::{Context, InputObject, Result, SimpleObject};
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use playa::user::repository::user::UpdateUserDto;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{User, UserError, UserErrorCode};

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
    pub async fn exec(ctx: &Context<'_>, id: Pxid, input: UserUpdateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let dto = UpdateUserDto {
            name: input.name,
            surname: input.surname,
        };

        match context.services.user.update(id.into_inner(), dto).await {
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
