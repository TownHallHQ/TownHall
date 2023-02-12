use async_graphql::{Context, InputObject, Result, SimpleObject};
use rand::distributions::Alphanumeric;
use rand::Rng;
use tracing::error;
use url::Url;

use crate::{
    context::SharedContext,
    modules::{
        auth::service::Token,
        link::{
            graphql::{Link, LinkError, LinkErrorCode},
            repository::CreateLinkDto,
        },
    },
    shared::repository::Repository,
};

#[derive(Debug, Default, SimpleObject)]
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
    pub fn exec(ctx: &Context<'_>, input: LinkCreateInput) -> Result<Self> {
        let context = ctx.data_unchecked::<SharedContext>();
        let user_claims = ctx.data_opt::<Token>().and_then(|jwt| {
            let Ok(claims) = context.services.auth.verify_token(jwt) else {
               error!("An error ocurred retriving claims from provided JWT");
               return None;
           };

            Some(claims)
        });

        let owner = if let Some(claims) = user_claims {
            context.repositories.user.find_by_key(claims.uid).unwrap()
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
            Some(owner.id)
        } else {
            None
        };

        let link = CreateLinkDto {
            original_url: input.url,
            custom_hash: Some(hash),
            owner_id,
        };

        let result = context.services.link.create(link);
        let response = Link::from(result);

        Ok(Self {
            link: Some(response),
            error: None,
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
