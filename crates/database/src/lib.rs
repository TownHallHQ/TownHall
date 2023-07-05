pub mod user;

use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use migration::sea_orm::{ConnectOptions, Database as SeaORMDatabase, DatabaseConnection, DbErr};
use migration::{Migrator, MigratorTrait};

use tracing::log;

#[derive(Clone)]
pub struct Database(Arc<DatabaseConnection>);

impl Deref for Database {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DatabaseConnection> for Database {
    fn from(value: DatabaseConnection) -> Self {
        let db_conn = Arc::new(value);

        Database(db_conn)
    }
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(database_url.to_owned());

        opt.max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db_conn = SeaORMDatabase::connect(opt).await?;
        let db_conn = Arc::new(db_conn);

        Ok(Self(db_conn))
    }

    pub async fn migrate(&self) -> Result<(), migration::DbErr> {
        Migrator::up(&*self.0, None).await
    }

    pub async fn refresh(&self) -> Result<(), migration::DbErr> {
        Migrator::fresh(&*self.0).await
    }

    pub async fn drop(&self) -> Result<(), migration::DbErr> {
        Migrator::down(&*self.0, None).await
    }
}
