use chrono::{DateTime, Utc};
use pxid::Pxid;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::shared::database::Database;

use super::error::{ImageError, Result};
use super::model::{Image, MimeType, UseCase};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImageRecord {
    pub id: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertImageDto {
    pub id: String,
    pub height: i32,
    pub width: i32,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub size: i32,
    pub provider_id: String,
    pub mime_type: MimeType,
    pub use_case: UseCase,
}

#[derive(Clone)]
pub struct ImageRepository {
    db: Database,
}

impl ImageRepository {
    pub fn new(db: &Database) -> Self {
        Self { db: db.to_owned() }
    }

    fn into_image_mime_type(mime_type: MimeType) -> entity::sea_orm_active_enums::ImageMimeType {
        use entity::sea_orm_active_enums::ImageMimeType;

        match mime_type {
            MimeType::Jpeg => ImageMimeType::ImageJpeg,
            MimeType::Png => ImageMimeType::ImagePng,
        }
    }

    fn into_mime_type(image_mime_type: entity::sea_orm_active_enums::ImageMimeType) -> MimeType {
        use entity::sea_orm_active_enums::ImageMimeType;

        match image_mime_type {
            ImageMimeType::ImageJpeg => MimeType::Jpeg,
            ImageMimeType::ImagePng => MimeType::Png,
        }
    }

    fn into_image_use_case(use_case: UseCase) -> entity::sea_orm_active_enums::ImageUseCase {
        use entity::sea_orm_active_enums::ImageUseCase;

        match use_case {
            UseCase::Avatar => ImageUseCase::Avatar,
            UseCase::Post => ImageUseCase::Post,
        }
    }

    fn into_use_case(image_mime_type: entity::sea_orm_active_enums::ImageUseCase) -> UseCase {
        use entity::sea_orm_active_enums::ImageUseCase;

        match image_mime_type {
            ImageUseCase::Avatar => UseCase::Avatar,
            ImageUseCase::Post => UseCase::Post,
        }
    }

    pub(crate) fn into_image_record(active_model: entity::image::Model) -> ImageRecord {
        ImageRecord {
            id: active_model.id,
            height: active_model.height,
            width: active_model.width,
            mime_type: Self::into_mime_type(active_model.mime_type),
            provider_id: active_model.provider_id,
            size: active_model.size,
            thumbnail_url: active_model.thumbnail_url,
            url: active_model.url,
            use_case: Self::into_use_case(active_model.use_case),
            created_at: DateTime::<Utc>::from_utc(active_model.created_at, Utc),
            updated_at: DateTime::<Utc>::from_utc(active_model.updated_at, Utc),
        }
    }

    pub async fn insert(&self, dto: InsertImageDto) -> Result<ImageRecord> {
        let image_active_model = entity::image::ActiveModel {
            id: Set(Image::new_id()?.to_string()),
            height: Set(dto.height),
            width: Set(dto.width),
            size: Set(dto.size),
            mime_type: Set(Self::into_image_mime_type(dto.mime_type)),
            use_case: Set(Self::into_image_use_case(dto.use_case)),
            url: Set(dto.url),
            thumbnail_url: Set(dto.thumbnail_url),
            provider_id: Set(dto.provider_id),
            ..Default::default()
        };

        let image_model = image_active_model.insert(&*self.db).await.map_err(|err| {
            tracing::error!(%err, "Failed to insert Image into the database");
            ImageError::RepositoryError
        })?;
        let image_record = Self::into_image_record(image_model);

        Ok(image_record)
    }

    pub async fn find_by_id(&self, id: Pxid) -> Result<Option<ImageRecord>> {
        let maybe_image = entity::prelude::Image::find()
            .filter(entity::image::Column::Id.eq(id.to_string()))
            .one(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to find Image by ID");
                ImageError::RepositoryError
            })?;

        if let Some(image_model) = maybe_image {
            return Ok(Some(Self::into_image_record(image_model)));
        }

        Ok(None)
    }

    pub async fn delete(&self, id: Pxid) -> Result<()> {
        entity::prelude::Image::delete_by_id(id.to_string())
            .exec(&*self.db)
            .await
            .map_err(|err| {
                tracing::error!(%err, "Failed to delete Image by ID");
                ImageError::RepositoryError
            })?;

        Ok(())
    }
}
