use async_trait::async_trait;
use pxid::Pxid;

use crate::image::error::{ImageError, Result};
use crate::image::model::{Image, MimeType, UseCase};
use crate::image::repository::ImageRepository;

use super::repository::InsertImageDto;

#[derive(Clone)]
pub struct ImageProviderResource {
    /// ID to use when identifying this Image Resource in the Provider
    pub id: String,
    pub size: i32,
    pub height: i32,
    pub width: i32,
    pub mime_type: MimeType,
    pub url: String,
    pub thumbnail_url: Option<String>,
}

#[async_trait]
pub trait ImageProvider {
    /// Uploads an image to the storage service
    async fn upload(&self, bytes: Vec<u8>) -> Result<ImageProviderResource>;

    /// Retrieves an image from the storage service
    async fn find_by_id(&self, id: &str) -> Result<Option<ImageProviderResource>>;

    /// Deletes an image in the storage service
    async fn delete(&self, id: &str) -> Result<Option<ImageProviderResource>>;
}

pub struct UploadImageDto {
    /// Bytes from the image file
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct ImageService<P: ImageProvider> {
    repository: Box<ImageRepository>,
    provider: Box<P>,
}

impl<P> ImageService<P>
where
    P: ImageProvider,
{
    pub fn new(repository: ImageRepository, provider: P) -> Self {
        Self {
            repository: Box::new(repository),
            provider: Box::new(provider),
        }
    }

    pub fn validate(&self, bytes: &[u8], use_case: UseCase) -> Result<()> {
        if bytes.is_empty() {
            return Err(ImageError::EmptyBytes);
        }

        if use_case.is_too_big(bytes.len()) {
            return Err(ImageError::ImageFileTooBig(
                bytes.len(),
                use_case.max_size_allowed(),
            ));
        }

        Ok(())
    }

    pub async fn upload(&self, dto: UploadImageDto, use_case: UseCase) -> Result<Image> {
        if dto.bytes.is_empty() {
            return Err(ImageError::EmptyBytes);
        }

        if use_case.is_too_big(dto.bytes.len()) {
            return Err(ImageError::ImageFileTooBig(
                dto.bytes.len(),
                use_case.max_size_allowed(),
            ));
        }

        let image_provider_resource = self.provider.upload(dto.bytes).await?;
        let insert_image_dto = InsertImageDto {
            id: Image::new_id()?.to_string(),
            height: image_provider_resource.height,
            width: image_provider_resource.width,
            mime_type: image_provider_resource.mime_type,
            size: image_provider_resource.size,
            url: image_provider_resource.url,
            thumbnail_url: image_provider_resource.thumbnail_url,
            use_case,
            provider_id: image_provider_resource.id,
        };
        let image_record = self.repository.insert(insert_image_dto).await?;
        let image = Image::try_from(image_record)?;

        Ok(image)
    }

    pub async fn find_by_id(&self, id: Pxid) -> Result<Option<Image>> {
        if let Some(image_record) = self.repository.find_by_id(id).await? {
            let image = Image::try_from(image_record)?;

            return Ok(Some(image));
        }

        Ok(None)
    }

    pub async fn delete(&self, id: Pxid) -> Result<()> {
        if let Some(image) = self.find_by_id(id).await? {
            self.repository.delete(id).await?;
            self.provider.delete(&image.provider_id).await?;

            return Ok(());
        }

        Err(ImageError::NotFound)
    }
}
