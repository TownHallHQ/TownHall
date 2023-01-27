use async_graphql::{Enum, SimpleObject, ID};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::modules::user::model::User as UserModel;

// enum Role {
//     User,
// }

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
            id: ID(value.id),
            name: value.name,
            surname: value.last_name,
            email: value.email,
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
