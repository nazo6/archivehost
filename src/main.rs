use tracing::info;

use crate::config::CONN;

mod cli;
mod common;
mod config;
#[allow(warnings, unused)]
mod db;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter("archivehost=debug")
        .pretty()
        .init();

    let _ = &*crate::config::CONFIG;

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

    cli::start().await?;

    Ok(())
}
