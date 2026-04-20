mod api;
mod core;
mod db;
mod models;
mod plugins;
mod reports;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting Autonomous Bug Bounty Recon Agent...");

    // Initialize Database
    let pool = db::init_db().await?;
    info!("Connected to PostgreSQL and ran migrations.");

    Ok(())
}
