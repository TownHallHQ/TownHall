use anyhow::{anyhow, bail, Result};
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use pxid::Pxid;

use me::{MeMeError, MeMeUser, Variables};

use crate::{DateTime, GRAPHQL_PATH};

use super::AuthClient;

#[derive(GraphQLQuery)]
#[graphql(
    response_derives = "Clone,Debug,Deserialize",
    schema_path = "schema.json",
    query_path = "src/modules/auth/me/Me.gql"
)]
pub struct Me {
    pub user: Option<MeMeUser>,
    pub error: Option<MeMeError>,
}

pub async fn me(auth_client: &AuthClient) -> Result<Me> {
    let url = auth_client.domain.join(GRAPHQL_PATH)?;
    let res = post_graphql::<Me, _>(&auth_client.client, url, Variables {})
        .await
        .map_err(|err| anyhow!("Failed to create token. {err}"))?;

    if let Some(ref data) = res.data {
        return Ok(Me {
            user: data.me.user.to_owned(),
            error: data.me.error.to_owned(),
        });
    }

    bail!("Failed to get user data. err = {:#?}", res.errors.unwrap())
}
