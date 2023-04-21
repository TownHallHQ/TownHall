use async_graphql::{ComplexObject, Context, Enum, SimpleObject, ID};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use quicklink::link::error::Result;

use crate::context::SharedContext;
use crate::services::auth::Token;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    InvalidEmail,
    EmailTaken,
    Unauthorized,
    Internal,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct UserError {
    pub code: UserErrorCode,
    pub message: String,
}

/// A Platform's User
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl User {
    /// Retrieves links for `User` instance, if and only if the `User` instance
    /// belongs to the currently authenticated user.
    pub async fn links(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<crate::graphql::modules::link::types::Link>> {
        let token = ctx.data_unchecked::<Token>();
        let context = ctx.data_unchecked::<SharedContext>();
        let claims = context.services.auth.verify_token(token).unwrap();

        if self.id.0.to_string() == claims.uid.to_string() {
            let links = context.services.link.find_by_owner_id(claims.uid).await?;

            return Ok(links
                .into_iter()
                .map(crate::graphql::modules::link::types::Link::from)
                .collect());
        }

        Ok(Vec::default())
    }
}

impl From<quicklink::user::model::User> for User {
    fn from(value: quicklink::user::model::User) -> Self {
        User {
            id: ID(value.id.to_string()),
            name: value.name,
            surname: value.surname,
            email: value.email.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
