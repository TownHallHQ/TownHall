use std::sync::Arc;
use std::time::Duration;

use anyhow::{Error, Result};

use crate::config::Config;
use crate::services::Services;

/// Wraps a `Context` into a `Arc` to allow sharing between treads
pub type SharedContext = Arc<Context>;

#[derive(Clone)]
pub struct Context {
    pub services: Services,
}

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let services = Services::new(config);

        Ok(Self { services })
    }
}
