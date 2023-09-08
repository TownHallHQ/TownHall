use std::sync::Arc;

use async_trait::async_trait;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use playa::image::error::{ImageError, Result};
use playa::image::model::MimeType;
use playa::image::service::{ImageProvider, ImageProviderResource};

const BUCKET_NAME: &str = "playa";
const MINIO_DEVELOPMENT_URL: &str = "http://127.0.0.1:9000";
const PXID_PREFIX: &str = "mnio";

#[derive(Clone)]
pub struct MinIOProvider {
    bucket: Arc<Bucket>,
    pxid: Arc<pxid::Factory>,
}

impl MinIOProvider {
    pub async fn new(user: &str, password: &str) -> Result<Self> {
        let pxid = Arc::new(pxid::Factory::new()?);
        let bucket = Self::get_or_create_bucket(user, password).await?;
        let bucket = Arc::new(bucket);

        Ok(Self { bucket, pxid })
    }

    async fn get_or_create_bucket(user: &str, password: &str) -> Result<Bucket> {
        let region = Region::Custom {
            region: String::default(),
            endpoint: MINIO_DEVELOPMENT_URL.to_string(),
        };
        let credentials = Credentials {
            access_key: Some(user.to_string()),
            secret_key: Some(password.to_string()),
            security_token: None,
            session_token: None,
            expiration: None,
        };

        let bucket = Bucket::new(BUCKET_NAME, region, credentials).map_err(|err| {
            ImageError::ProviderError(format!("Failed to create bucket: {}", err))
        })?;

        Ok(bucket.with_path_style())
    }

    #[inline]
    fn resource_url(id: &str) -> String {
        format!("{}/{}/{}", MINIO_DEVELOPMENT_URL, BUCKET_NAME, id)
    }
}

#[async_trait]
impl ImageProvider for MinIOProvider {
    async fn upload(&self, bytes: Vec<u8>) -> Result<ImageProviderResource> {
        let id = self.pxid.new_id(PXID_PREFIX)?.to_string();
        let _ = self
            .bucket
            .put_object_with_content_type(id.to_string(), &bytes, &MimeType::Jpeg.to_string())
            .await
            .unwrap();

        Ok(ImageProviderResource {
            id: id.to_string(),
            size: bytes.len() as i32,
            height: 0,
            width: 0,
            mime_type: MimeType::Jpeg,
            url: Self::resource_url(&id),
            thumbnail_url: None,
        })
    }

    async fn find_by_id(&self, _id: &str) -> Result<Option<ImageProviderResource>> {
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<Option<ImageProviderResource>> {
        self.bucket.delete_object(id).await.map_err(|err| {
            tracing::error!("Failed to delete object: {}", err);
            ImageError::ProviderError(format!("Failed to delete object: {}", err))
        })?;

        Ok(None)
    }
}
