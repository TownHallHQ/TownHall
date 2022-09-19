use anyhow::{Error, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    db: DatabaseConnection,
}

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let db = Self::make_database_connection(&config).await?;

        Ok(Self { db })
    }

    pub fn conn(&self) -> DatabaseConnection {
        self.db.clone()
    }

    async fn make_database_connection(config: &Config) -> Result<DatabaseConnection> {
        let mut opt = ConnectOptions::new(config.database_url.clone());

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        Database::connect(opt)
            .await
            .map_err(|err| Error::msg(err.to_string()))
    }
}
