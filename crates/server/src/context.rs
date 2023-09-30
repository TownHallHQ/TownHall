use std::sync::Arc;

use anyhow::Result;

use crate::config::Config;
use crate::services::{Services, SharedServices};

#[derive(Clone)]
pub struct Context {
    pub services: SharedServices,
}

pub type SharedContext = Arc<Context>;

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let services = Services::shared(config).await?;

        Ok(Self { services })
    }
}
