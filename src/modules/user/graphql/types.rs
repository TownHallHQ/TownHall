use std::str::from_utf8;

use async_graphql::{ComplexObject, Context, Enum, SimpleObject, ID};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::context::SharedContext;
use crate::modules::link::graphql::Link;
use crate::shared::repository::Repository;

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum UserErrorCode {
    EmailTaken,
    Unauthorized,
    Unknown,
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

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub links_ids: Vec<ID>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[ComplexObject]
impl User {
    pub async fn links(&self, ctx: &Context<'_>) -> Vec<Link> {
        let context = ctx.data_unchecked::<SharedContext>();
        self.links_ids
            .iter()
            .map(|link_id| {
                let link = context
                    .repositories
                    .link
                    .find_by_key(link_id.as_bytes())
                    .unwrap()
                    .unwrap();

                Link::from(link)
            })
            .collect::<Vec<Link>>()
    }
}

impl From<crate::modules::user::model::User> for User {
    fn from(value: crate::modules::user::model::User) -> Self {
        User {
            // FIXME: This is very expensive
            id: ID(from_utf8(&value.id).unwrap().to_string()),
            name: value.name,
            surname: value.surname,
            email: value.email,
            links_ids: value
                .links_ids
                .iter()
                .map(|link_id| ID::from(from_utf8(link_id).unwrap()))
                .collect::<Vec<ID>>(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
