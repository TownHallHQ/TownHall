use std::io::Read;

use async_graphql::types::Upload;
use async_graphql::{Context, Result, SimpleObject};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use townhall::user::service::UploadAvatarDto;

use crate::context::SharedContext;
use crate::graphql::modules::user::types::{UserError, UserErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct UserAvatarUpdate {
    user: Option<crate::graphql::modules::user::types::User>,
    error: Option<crate::graphql::modules::user::types::UserError>,
}

impl UserAvatarUpdate {
    #[instrument(skip_all, fields(user_id))]
    pub async fn exec(ctx: &Context<'_>, file: Upload) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_id = ctx.data_unchecked::<Token>().user_id();

        let Ok(upload_value) = file.value(ctx) else {
            tracing::error!("Failed to upload avatar file.");

            return Ok(UserAvatarUpdate {
                user: None,
                error: Some(UserError {
                    code: UserErrorCode::AvatarUploadError,
                    message: String::from("Failed to upload avatar, try again later"),
                }),
            });
        };

        let user = context.services.user.find_by_id(user_id).await?;

        if user.is_none() {
            return Ok(UserAvatarUpdate {
                user: None,
                error: Some(UserError {
                    code: UserErrorCode::Unauthorized,
                    message: String::from("Failed to upload avatar, try again later"),
                }),
            });
        }

        let mut bytes: Vec<u8> = Vec::new();
        let Ok(_bytes_of_data) = upload_value.into_read().read_to_end(&mut bytes) else {
            tracing::error!("Failed to read bytes from file.");

            return Ok(UserAvatarUpdate {
                user: None,
                error: Some(UserError {
                    code: UserErrorCode::AvatarUploadError,
                    message: String::from("Failed to upload avatar, try again later"),
                }),
            });
        };

        match context
            .services
            .user
            .update_avatar(user_id, UploadAvatarDto { bytes })
            .await
        {
            Ok(user) => Ok(UserAvatarUpdate {
                user: Some(user.into()),
                error: None,
            }),
            Err(err) => {
                tracing::error!(?err, "Failed to update avatar");

                Ok(UserAvatarUpdate {
                    user: None,
                    error: Some(UserError {
                        code: UserErrorCode::AvatarUploadError,
                        message: String::from("Failed to upload avatar, try again later"),
                    }),
                })
            }
        }
    }
}
