use anyhow::Result;
use clap::{Parser, Subcommand};
use sea_orm_cli::commands::generate::run_generate_command;
use sea_orm_cli::{DateTimeCrate, GenerateSubcommands};

use migration::{Migrator, MigratorTrait};
use townhall::shared::database::Database;

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
    /// Generates Database Entities
    Entities(DatabaseArgs),
}

impl DatabseSub {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Self::Migrate(opt) => {
                tracing::info!("Creating new Database Manager");
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::info!("Running database migrations");
                Migrator::up(&*manager, None).await.unwrap();
            }
            Self::Refresh(opt) => {
                tracing::info!("Creating new Database Manager");
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::warn!("Refreshing database instance");
                Migrator::fresh(&*manager).await.unwrap();
            }
            Self::Drop(opt) => {
                tracing::info!("Creating new Database Manager");
                let manager = Database::new(&opt.database_url).await.unwrap();

                tracing::info!("Dropping Database Tables");
                Migrator::down(&*manager, None).await.unwrap();
            }
            Self::Entities(opt) => {
                let opts = GenerateSubcommands::Entity {
                    compact_format: false,
                    expanded_format: false,
                    include_hidden_tables: false,
                    tables: Vec::default(),
                    ignore_tables: Vec::default(),
                    max_connections: 1,
                    output_dir: String::from("./crates/entity/src"),
                    database_schema: String::from("public"),
                    database_url: opt.database_url.clone(),
                    with_serde: String::from("both"),
                    serde_skip_deserializing_primary_key: false,
                    serde_skip_hidden_column: false,
                    with_copy_enums: false,
                    date_time_crate: DateTimeCrate::Chrono,
                    lib: true,
                    model_extra_derives: Vec::default(),
                    model_extra_attributes: Vec::default(),
                    seaography: false,
                };

                tracing::info!("Generating Database Entities");
                run_generate_command(opts, true).await.unwrap();
            }
        }

        Ok(())
    }
}
