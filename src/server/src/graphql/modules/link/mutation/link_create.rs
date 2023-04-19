use async_graphql::{Context, InputObject, Result, SimpleObject};
use serde::{Deserialize, Serialize};

use tracing::instrument;

use quicklink::link::service::CreateLinkDto;

use crate::context::SharedContext;

#[derive(Debug, Default, InputObject)]
pub struct LinkCreateInput {
    pub url: String,
    pub ulid: Option<String>,
}

impl From<LinkCreateInput> for CreateLinkDto {
    fn from(value: LinkCreateInput) -> Self {
        Self {
            ulid: value.ulid,
            original_url: value.url,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, SimpleObject)]
pub struct LinkCreate {
    link: Option<crate::graphql::modules::link::types::Link>,
    error: Option<crate::graphql::modules::link::types::LinkError>,
}

impl LinkCreate {
    #[instrument(skip_all, fields(url = %input.url))]
    pub async fn exec(ctx: &Context<'_>, input: LinkCreateInput) -> Result<Self> {
        use crate::graphql::modules::link::types::Link;
        use crate::graphql::modules::link::types::LinkError;

        let context = ctx.data_unchecked::<SharedContext>();
        let dto = CreateLinkDto::from(input);

        match context.services.link.create(dto).await {
            Ok(link) => Ok(Self {
                link: Some(Link::from(link)),
                error: None,
            }),
            Err(err) => Ok(Self {
                link: None,
                error: Some(LinkError::from(err)),
            }),
        }
    }
}
