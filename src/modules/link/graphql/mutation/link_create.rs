use async_graphql::{Context, InputObject, Result, SimpleObject};
use rand::distributions::Alphanumeric;
use rand::Rng;

use url::Url;

use crate::context::SharedContext;
use crate::modules::auth::service::Token;
use crate::modules::link::graphql::{Link, LinkError, LinkErrorCode};
use crate::modules::link::repository::CreateLinkDto;
use crate::shared::repository::Repository;

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
        let jwt = ctx.data_opt::<Token>().unwrap();
        let claims = context.services.auth.verify_token(jwt).unwrap();
        let user_id = claims.uid.as_bytes().to_vec();

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
            if context
                .repositories
                .link
                .find_by_key(custom_hash.as_bytes())
                .unwrap()
                .is_some()
            {
                return Ok(Self {
                    link: None,
                    error: Some(LinkError {
                        code: LinkErrorCode::CustomHashUsed,
                        message: format!("Link with hash \"{custom_hash}\" already exists",),
                    }),
                });
            }

            custom_hash
        } else {
            LinkCreate::create_hash()
        };

        let link = CreateLinkDto {
            original_url: input.url,
            custom_hash: Some(hash.clone()),
            owner_id: user_id.clone(),
        };
        let record = context
            .repositories
            .link
            .create_with_key(hash.as_bytes(), link)
            .unwrap();
        let link_id = record.id.clone();

        context
            .repositories
            .user
            .update(
                user_id,
                Box::new(|mut user| {
                    user.links_ids.push(link_id);

                    Ok(user)
                }),
            )
            .unwrap();

        Ok(Self {
            link: Some(Link::from(record)),
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
