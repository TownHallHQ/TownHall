use async_graphql::SimpleObject;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub email: String,
    #[graphql(visible = false)]
    pub hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
