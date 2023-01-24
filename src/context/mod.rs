mod config;
mod services;
mod store;

pub use config::Config;
pub use services::Services;
pub use store::Store;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::config::Config;

use self::services::Services;
use self::store::Store;

/// Wraps a `Context` into a `Arc` to allow sharing between treads
pub type SharedContext = Arc<Context>;

#[derive(Clone)]
pub struct Context {
    pub services: Services,
}

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let db_path = PathBuf::from(config.database_path.clone());
        let store = Store::new(db_path);
        let services = Services::new(config, store);

        Ok(Self { services })
    }
}
