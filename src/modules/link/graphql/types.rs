use std::str::from_utf8;

use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{modules::link::model::Link, shared::gql::TypeGQL};

#[derive(Copy, Clone, Debug, Deserialize, Enum, Eq, PartialEq, Serialize)]
pub enum LinkErrorCode {
    CustomHashUsed,
    InvalidUrl,
    Unauthorized,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct LinkError {
    pub code: LinkErrorCode,
    pub message: String,
}

/*#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Link {
    pub id: String,
    pub original_url: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<LinkModel> for Link {
    fn from(value: LinkModel) -> Self {
        Link {
            // FIXME: this is very expensive
            id: from_utf8(&value.id).unwrap().to_string(),
            original_url: value.original_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct TypeGQL<T>(pub T);*/

#[async_graphql::Object(name = "Link")]
impl TypeGQL<Link> {
    async fn id(&self) -> String {
        from_utf8(&self.0.id).unwrap().to_string()
    }

    async fn original_url(&self) -> &String {
        &self.0.original_url
    }

    async fn created_at(&self) -> DateTime<Local> {
        self.0.created_at
    }

    async fn updated_at(&self) -> DateTime<Local> {
        self.0.updated_at
    }
}
