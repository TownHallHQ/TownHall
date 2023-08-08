use std::str::FromStr;

use chrono::{DateTime, Utc};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use crate::image::error::{ImageError, Result};
use crate::image::repository::ImageRecord;

use super::{MimeType, UseCase};

pub const IMAGE_PXID_PREFIX: &str = "img";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Image {
    pub id: Pxid,
    pub height: i32,
    pub width: i32,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub size: i32,
    pub mime_type: MimeType,
    pub use_case: UseCase,
    pub provider_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Image {
    pub fn new_id() -> Result<Pxid> {
        Pxid::new(IMAGE_PXID_PREFIX).map_err(ImageError::PxidError)
    }
}

impl TryFrom<ImageRecord> for Image {
    type Error = ImageError;

    fn try_from(value: ImageRecord) -> Result<Self> {
        Ok(Image {
            id: Pxid::from_str(&value.id)?,
            height: value.height,
            width: value.width,
            url: value.url,
            thumbnail_url: value.thumbnail_url,
            size: value.size,
            mime_type: value.mime_type,
            use_case: value.use_case,
            provider_id: value.provider_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
