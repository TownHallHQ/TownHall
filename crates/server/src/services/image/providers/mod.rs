pub mod minio;

use std::sync::Arc;

use async_trait::async_trait;

use townhall::image::error::Result;
use townhall::image::service::{ImageProvider, ImageProviderResource};

use crate::services::Config;

pub const MINIO_ROOT_USER: &str = "MINIO_ROOT_USER";
pub const MINIO_ROOT_PASSWORD: &str = "MINIO_ROOT_PASSWORD";
pub const MINIO_ACCESS_KEY: &str = "MINIO_ACCESS_KEY";
pub const MINIO_SECRET_KEY: &str = "MINIO_SECRET_KEY";

#[derive(Clone)]
pub struct ImageServiceProvider(Arc<dyn ImageProvider + Send + Sync>);

impl ImageServiceProvider {
    /// Creates a new ImageServiceProvider based on the environment.
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        use self::minio::MinIOProvider;

        tracing::info!("Using Minio as Image Provider");
        let minio = MinIOProvider::new(&config.minio_username, &config.minio_password).await?;

        Ok(Self(Arc::new(minio)))
    }
}

#[async_trait]
impl ImageProvider for ImageServiceProvider {
    async fn upload(&self, bytes: Vec<u8>) -> Result<ImageProviderResource> {
        self.0.upload(bytes).await
    }

    async fn find_by_id(&self, _id: &str) -> Result<Option<ImageProviderResource>> {
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<Option<ImageProviderResource>> {
        self.0.delete(id).await
    }
}
