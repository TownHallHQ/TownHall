use async_graphql::{Context, Error, InputObject, Result, SimpleObject};
use chrono::{prelude::*, Duration};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::error;
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
    pub custom_hash: Option<String>,
}

impl LinkCreate {
    pub async fn exec(ctx: &Context<'_>, input: LinkCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_claims = ctx.data_opt::<Token>().and_then(|jwt| {
            let Ok(claims) = context.services.auth.verify_token(jwt) else {
                error!("An error ocurred retriving claims from provided JWT");
                return None;
            };

            Some(claims)
        });
        let owner = if let Some(claims) = user_claims {
            UserEntity::find()
                .filter(entity::user::Column::Id.eq(claims.uid))
                .one(&context.conn())
                .await
                .map_err(|db_err| Error::new(db_err.to_string()))?
        } else {
            None
        };

        if let Err(parse_error) = Url::parse(&input.url) {
            return Ok(Self {
                link: None,
                error: Some(LinkError {
                    code: LinkErrorCode::InvalidUrl,
                    message: parse_error.to_string(),
                }),
            });
        }

        let hash: String = if let Some(custom_hash) = input.custom_hash {
            custom_hash
        } else {
            LinkCreate::create_hash()
        };

        let owner_id = if let Some(owner) = owner {
            Set(Some(owner.id))
        } else {
            NotSet
        };

        let expires_at: DateTime<Utc> = Utc::now() + Duration::days(10);
        let naive_expires_at = expires_at.naive_utc();
        let link = entity::link::ActiveModel {
            id: NotSet,
            hash: Set(hash),
            original_url: Set(input.url),
            expires_at: Set(naive_expires_at),
            owner_id,
            created_at: Set(DateTime::default()),
            updated_at: Set(DateTime::default()),
        };

        match link.save(&context.conn()).await {
            Ok(created_link) => Ok(Self {
                link: Some(Link {
                    id: created_link.id.unwrap(),
                    hash: created_link.hash.unwrap(),
                    original_url: created_link.original_url.unwrap(),
                    expires_at: DateTime::default(),
                }),
                error: None,
            }),
            Err(db_err) => Err(Error::new(db_err.to_string())),
        }
    }

    fn create_hash() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
    }
}
