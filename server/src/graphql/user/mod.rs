use async_graphql::{Context, Enum, Object, Result, SimpleObject};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{self, prelude::User as UserEntity};

use crate::context::SharedContext;
use crate::services::auth::Token;

pub struct UserQueryRoot;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct UserError {
    code: UserErrorCode,
    message: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct User {
    id: i32,
    name: String,
    last_name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
struct Me {
    user: Option<User>,
    error: Option<UserError>,
}

#[Object]
impl UserQueryRoot {
    async fn me<'a>(&self, ctx: &'a Context<'_>) -> Result<Me> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();

            if let Some(user) = UserEntity::find()
                .filter(entity::user::Column::Id.eq(claims.uid))
                .one(&context.conn())
                .await
                .unwrap()
            {
                return Ok(Me {
                    user: Some(User {
                        id: user.id,
                        email: user.email,
                        name: user.name,
                        last_name: user.last_name,
                    }),
                    error: None,
                });
            }
        }

        Ok(Me {
            user: None,
            error: Some(UserError {
                code: UserErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }
}
