use config::CLI;
use tracing::{info, warn};

use crate::config::CONN;

mod cli;
mod common;
mod config;
#[allow(warnings, unused)]
mod db;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::INFO);

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_env_filter("archivehost=debug");

    subscriber.pretty().init();

    let _ = &*crate::config::CONFIG;

    if CLI.skip_migration {
        warn!("ATTENTION: Skipping migration. This is for development. Using this in production may cause data loss.");
    } else {
        #[cfg(debug_assertions)]
        {
            info!("Migrating in debug mode: forcing reset");
            CONN._db_push().accept_data_loss().force_reset().await?;
        }
        #[cfg(not(debug_assertions))]
        {
            info!("Migrating in release mode");
            CONN._migrate_deploy().await?;
        }
    }

    cli::start().await?;

    Ok(())
}
