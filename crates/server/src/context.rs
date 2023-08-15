use std::sync::Arc;

use crate::config::Config;
use crate::services::image::providers::minio::MinIOProvider;
use crate::services::{Services, SharedServices};

#[derive(Clone)]
pub struct Context {
    pub services: SharedServices<MinIOProvider>,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub async fn new(config: &Config) -> Self {
        let minio_provider = MinIOProvider::new(&config.minio_username, &config.minio_password)
            .await
            .expect("Failed to create MinIO provider");
        let services = Services::shared(config, minio_provider).await;

        Self { services }
    }

    pub async fn shared(config: &Config) -> SharedContext {
        let context = Self::new(config).await;

        Arc::new(context)
    }
}
