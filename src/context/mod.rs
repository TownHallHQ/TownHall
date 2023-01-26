mod config;
mod services;
mod store;

pub use config::Config;
pub use services::Services;
pub use store::Store;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

/// Wraps a `Context` into a `Arc` to allow sharing between treads
pub type SharedContext = Arc<Context>;

#[derive(Clone)]
pub struct Context {
    pub services: Arc<Services>,
}

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let db_path = PathBuf::from(config.database_path.clone());
        let store = Arc::new(Store::new(db_path));
        let services = Arc::new(Services::new(store, config));

        Ok(Self { services })
    }
}
