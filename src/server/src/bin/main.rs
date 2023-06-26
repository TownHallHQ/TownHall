use dotenv::dotenv;
use tracing::info;

use libserver::start;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        // When running on development mode, its convenient to read environment
        // variables from a `.env` file.
        info!("Reading environment variables from .env file");
        dotenv().ok();
    }

    tracing_subscriber::fmt::init();

    start().await;
}
