use clap::Parser;
use dotenv::dotenv;
use tracing::info;

use libcli::database::DatabseSub;
use libserver::start;

const ABOUT: &str = "gabble Application Command Line Interface";

#[derive(Debug, Parser)]
#[command(next_line_help = true)]
#[command(name = "gabble", author, version, about, long_about = Some(ABOUT))]
pub enum Cli {
    /// Manage Database
    #[clap(subcommand)]
    Database(DatabseSub),
    /// Initializes and starts Gabble Server
    Serve,
}

impl Cli {
    pub async fn exec(self) {
        match self {
            Self::Database(cmd) => cmd.exec().await,
            Self::Serve => start().await,
        }
    }
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        // When running on development mode, its convenient to read environment
        // variables from a `.env` file.
        info!("Reading environment variables from .env file");
        dotenv().ok();
    }

    tracing_subscriber::fmt::init();

    let cli_opts = Cli::parse();

    cli_opts.exec().await;
}
