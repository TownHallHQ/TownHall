use clap::{Parser, Subcommand};

use database::Database;

use crate::config::DATABASE_URL_ENV_VAR;

#[derive(Clone, Debug, Parser)]
#[command(next_line_help = true)]
pub struct DatabaseArgs {
    #[clap(long = "database-url", env = DATABASE_URL_ENV_VAR)]
    pub database_url: String,
}

#[derive(Clone, Debug, Subcommand)]
pub enum DatabseSub {
    /// Runs all pending database migrations
    Migrate(DatabaseArgs),
    /// Drops and recreates the database, then runs migrations
    Refresh(DatabaseArgs),
    /// Drops migrations using drop
    Drop(DatabaseArgs),
}

impl DatabseSub {
    pub async fn exec(&self) {
        match self {
            Self::Migrate(opt) => {
                tracing::info!("Creating new Database Manager");
                tracing::info!("{}", opt.database_url);
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::info!("Running database migrations");
                manager.migrate().await.unwrap();
            }
            Self::Refresh(opt) => {
                tracing::info!("Creating new Database Manager");
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::warn!("Refreshing database instance");
                manager.refresh().await.unwrap();
            }
            Self::Drop(opt) => {
                tracing::info!("Creating new Database Manager");
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::info!("Dropping Database Tables");
                manager.drop().await.unwrap();
            }
        }
    }
}
