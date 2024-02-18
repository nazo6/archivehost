use config::CONFIG;
use db::migration::{Migrator, MigratorTrait};
use eyre::Context;
use sea_orm::Database;
use tracing::{info, warn};

mod app;
mod cli;
mod common;
mod config;
mod constant;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter("archivehost=info");

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_env_filter("archivehost=debug");

    subscriber.pretty().init();

    let _ = &*crate::config::CONFIG;

    let conn = {
        let url = format!(
            "sqlite://{}/db.sqlite?mode=rwc",
            CONFIG.root.to_string_lossy()
        );
        let conn = Database::connect(url)
            .await
            .wrap_err("Failed to connect to database")?;

        if cli::CLI.skip_migration {
            warn!("ATTENTION: Skipping migration. This is for development. Using this in production may cause data loss.");
        } else {
            let pending = Migrator::get_pending_migrations(&conn).await?;
            if !pending.is_empty() {
                info!("Running pending migrations");
                Migrator::up(&conn, None).await?;
            }
        }
        conn
    };

    app::start(conn).await?;

    Ok(())
}
