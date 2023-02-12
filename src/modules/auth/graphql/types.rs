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
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        User {
            // FIXME: This is very expensive
            id: ID(from_utf8(&value.id).unwrap().to_string()),
            name: value.name,
            surname: value.last_name,
            email: value.email,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
