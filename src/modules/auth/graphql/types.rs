use async_graphql::{Enum, SimpleObject, ID};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

enum Role {
    User,
}

/// Platform User. A platform user may have priviledges for different
/// operations based on its `Role`.
#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: Option<String>,
    // pub role: Enum<Role>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

// impl From<entity::user::Model> for User {
//     fn from(model: entity::user::Model) -> Self {
//         User {
//             id: ID(model.id.to_string()),
//             email: model.email,
//             name: model.name,
//             surname: model.surname,
//             phone: model.phone,
//             role: model.role.into(),
//             created_at: model.created_at.into(),
//             updated_at: model.updated_at.into(),
//         }
//     }
// }
