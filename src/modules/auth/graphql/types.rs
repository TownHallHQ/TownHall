use std::str::from_utf8;

use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::modules::user::model::User as UserModel;

/// Platform User. A platform user may have priviledges for different
/// operations based on its `Role`.
#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub links_ids: Vec<ID>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
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
