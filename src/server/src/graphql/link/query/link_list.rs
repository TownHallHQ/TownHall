use async_graphql::{Context, Result, SimpleObject};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{self, prelude::Link as LinkEntity};

use crate::context::SharedContext;
use crate::graphql::link::{Link, LinkError, LinkErrorCode};
use crate::services::auth::Token;

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct LinkList {
    links: Option<Vec<Link>>,
    error: Option<LinkError>,
}

impl LinkList {
    pub async fn exec(ctx: &Context<'_>) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();

        if let Some(jwt) = ctx.data_opt::<Token>() {
            let claims = context.services.auth.verify_token(jwt).unwrap();
            let links = LinkEntity::find()
                .filter(entity::link::Column::OwnerId.eq(claims.uid))
                .all(&context.conn())
                .await
                .unwrap()
                .iter()
                .map(|link| Link {
                    id: link.id,
                    hash: link.hash.clone(),
                    expires_at: link.expires_at,
                    original_url: link.original_url.clone(),
                })
                .collect();

            return Ok(LinkList {
                links: Some(links),
                error: None,
            });
        }

        Ok(Self {
            links: None,
            error: Some(LinkError {
                code: LinkErrorCode::Unauthorized,
                message: String::from("You must provide a token to access this resource"),
            }),
        })
    }
}
