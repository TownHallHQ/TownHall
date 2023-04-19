use std::sync::Arc;

use crate::config::Config;
use crate::services::{Services, SharedServices};

#[derive(Clone)]
pub struct Context {
    pub services: SharedServices,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub async fn new(config: Config) -> Self {
        let services = Services::shared(&config).await;

        Self { services }
    }

    pub async fn shared(config: Config) -> SharedContext {
        let context = Self::new(config).await;

        Arc::new(context)
    }
}
