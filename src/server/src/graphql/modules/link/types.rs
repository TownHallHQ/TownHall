use async_graphql::{Enum, SimpleObject, ID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct LinkError {
    pub code: LinkErrorCode,
    pub message: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Enum, PartialEq, Eq)]
pub enum LinkErrorCode {
    UlidParse,
    Pxid,
    Internal,
}

impl From<quicklink::link::error::LinkError> for LinkErrorCode {
    fn from(error: quicklink::link::error::LinkError) -> Self {
        use quicklink::link::error::LinkError as CoreLinkError;

        match error {
            CoreLinkError::UlidParseError(_) => Self::UlidParse,
            CoreLinkError::PxidError(_) => Self::Pxid,
            _ => {
                tracing::warn!("Found unknown error: {error:?}");
                LinkErrorCode::Internal
            }
        }
    }
}

impl From<quicklink::link::error::LinkError> for LinkError {
    fn from(error: quicklink::link::error::LinkError) -> Self {
        let message = error.to_string();
        let code = LinkErrorCode::from(error);

        Self { code, message }
    }
}

/// A Link record used to redirect to the underlying `original_url`
#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Link {
    pub id: ID,
    pub ulid: String,
    pub original_url: String,
}

impl From<quicklink::link::model::link::Link> for Link {
    fn from(value: quicklink::link::model::link::Link) -> Self {
        Self {
            id: ID(value.id.to_string()),
            ulid: value.ulid.to_string(),
            original_url: value.original_url.to_string(),
        }
    }
}
