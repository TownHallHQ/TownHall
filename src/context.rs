use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::Path;

use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    db: PgPool,
}

impl Context {
    pub async fn new(config: Config) -> Self {
        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await
            .expect("Failed to establish a connection with the database");

        Self { db }
    }

    pub async fn bootstrap(&self) {
        self.run_migrations().await;
    }

    pub fn conn(&self) -> PgPool {
        self.db.clone()
    }

    async fn run_migrations(&self) {
        let path = Path::new("./migrations");
        let migrator = Migrator::new(path).await.unwrap();

        migrator.run(&self.db).await;
    }
}
