use config::CLI;
use migration::{Migrator, MigratorTrait as _};
use tracing::{info, warn};

use crate::config::CONN;

mod cli;
mod common;
mod config;

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
        info!("Database migration running");
        Migrator::up(&*CONN, None).await?;
    }

    cli::start().await?;

    Ok(())
}
