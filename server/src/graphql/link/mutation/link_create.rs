use async_graphql::{Context, InputObject, Result, SimpleObject};
use chrono::{prelude::*, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use url::Url;

use entity::{self, prelude::User as UserEntity};

use crate::context::SharedContext;
use crate::graphql::link::{Link, LinkError, LinkErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct LinkCreate {
    link: Option<Link>,
    error: Option<LinkError>,
}

#[derive(Debug, Default, InputObject)]
pub struct LinkCreateInput {
    pub url: String,
}

impl LinkCreate {
    pub async fn exec(ctx: &Context<'_>, input: LinkCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();

            if let Some(user) = UserEntity::find()
                .filter(entity::user::Column::Id.eq(claims.uid))
                .one(&context.conn())
                .await
                .unwrap()
            {
                if let Err(parse_error) = Url::parse(&input.url) {
                    return Ok(Self {
                        link: None,
                        error: Some(LinkError {
                            code: LinkErrorCode::InvalidUrl,
                            message: format!("Provided URL is not valid: {}", input.url),
                        }),
                    });
                }

                let expires_at: DateTime<Utc> = Utc::now() + Duration::days(10);
                let naive_expires_at = expires_at.naive_utc();
                let link = entity::link::ActiveModel {
                    id: NotSet,
                    hash: Set(LinkCreate::create_hash()),
                    original_url: Set(input.url),
                    expires_at: Set(naive_expires_at),
                    owner_id: NotSet,
                    created_at: NotSet,
                    updated_at: NotSet,
                };

                if let Ok(created_link) = link.save(&context.conn()).await {
                    let expires_at = created_link.expires_at.unwrap();

                    return Ok(Self {
                        link: Some(Link {
                            id: created_link.id.unwrap(),
                            hash: created_link.hash.unwrap(),
                            original_url: created_link.original_url.unwrap(),
                            expires_at: created_link.original_url.unwrap(),
                            owner_id: created_link.owner_id.unwrap(),
                        }),
                        error: None,
                    });
                };

                return Ok(Self {
                    link: None,
                    error: Some(LinkError {
                        code: LinkErrorCode::Unauthorized,
                        message: String::from("Invalid token provided"),
                    }),
                });
            }
        }

        Ok(Self {
            link: None,
            error: Some(LinkError {
                code: LinkErrorCode::Unauthorized,
                message: String::from("Invalid token provided"),
            }),
        })
    }

    fn create_hash() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
    }
}
