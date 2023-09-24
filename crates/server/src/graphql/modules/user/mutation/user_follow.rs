use async_graphql::{Context, Result, SimpleObject};
use pxid::graphql::Pxid;
use serde::{Deserialize, Serialize};

use playa::user::service::FollowPeers;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{UserError, UserErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserFollow {
    error: Option<UserError>,
}

impl UserFollow {
    pub async fn exec(ctx: &Context<'_>, followee_id: Pxid) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_id = ctx.data_unchecked::<Token>().user_id();

        match context
            .services
            .user
            .follow(FollowPeers {
                follower_id: user_id,
                followee_id: followee_id.into_inner(),
            })
            .await
        {
            Ok(_) => Ok(Self { error: None }),
            Err(err) => Ok(Self {
                error: Some(UserError {
                    code: UserErrorCode::Internal,
                    message: format!("An error ocurred: {err}"),
                }),
            }),
        }
    }
}
